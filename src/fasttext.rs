//! FastText `.bin` model reader.
//!
//! [`FastTextEmbedding`] loads Facebook FastText binary models (the format
//! produced by the `fasttext` CLI and used by roshan-research's
//! `fasttext_skipgram_300.bin` Persian embedding).
//!
//! Supported features:
//! - In-vocabulary word vectors (direct matrix row lookup)
//! - Subword inference for OOV words via character n-gram hashing
//! - `most_similar`, `doesnt_match`, and the full [`WordEmbedding`] trait
//!
//! Quantized models are not supported and return a [`Error::Parse`] error.
//!
//! # Example
//!
//! ```no_run
//! use std::path::Path;
//! use rustam::fasttext::FastTextEmbedding;
//! use rustam::embedding::WordEmbedding;
//!
//! let model = FastTextEmbedding::from_file(Path::new("fasttext_skipgram_300.bin")).unwrap();
//! let v = model.embed_word("کتاب");        // in-vocabulary
//! let v2 = model.embed_word("کتاب‌خانه"); // OOV — uses subword n-grams
//! let sim = model.most_similar("ایران", 5);
//! ```

use std::collections::HashMap;
use std::io::{BufReader, Read};
use std::path::Path;

use crate::embedding::WordEmbedding;
use crate::error::{Error, Result};

const MAGIC: u32 = 793712314;
const VERSION: u32 = 12;

// Binary args block is 72 bytes; byte offsets for fields we need:
// offset 12 → dim (i32), offset 48 → bucket (i32),
// offset 52 → minn (i32), offset 56 → maxn (i32)
const ARGS_NBYTES: usize = 72;

/// Word embeddings loaded from a Facebook FastText `.bin` model file.
///
/// Word vectors include subword (character n-gram) contributions, making the
/// model robust to morphologically rich Persian text including OOV words.
/// Vectors returned by [`embed_word`](Self::embed_word) are L2-normalised.
pub struct FastTextEmbedding {
    vocab: Vec<String>,
    /// Maps word → dictionary position (= row index in `input_matrix`).
    index: HashMap<String, usize>,
    /// Full input matrix: (nwords + bucket) × dim, row-major, raw (not normalised).
    input_matrix: Vec<f32>,
    /// Pre-computed L2 norms of each word row for fast cosine similarity.
    word_norms: Vec<f32>,
    nwords: usize,
    bucket: usize,
    dim: usize,
    minn: usize,
    maxn: usize,
}

impl FastTextEmbedding {
    /// Loads a FastText `.bin` model from `path`.
    pub fn from_file(path: &Path) -> Result<Self> {
        let f = std::fs::File::open(path)?;
        let mut r = BufReader::new(f);

        // --- header ---
        let magic = read_u32(&mut r)?;
        if magic != MAGIC {
            return Err(Error::Parse(format!(
                "not a FastText binary (magic={magic:#010x}, expected {MAGIC:#010x})"
            )));
        }
        let version = read_u32(&mut r)?;
        if version != VERSION {
            return Err(Error::Parse(format!(
                "unsupported FastText version {version}, expected {VERSION}"
            )));
        }

        // --- args block ---
        let mut args = [0u8; ARGS_NBYTES];
        r.read_exact(&mut args)?;
        let dim    = i32::from_le_bytes(args[12..16].try_into().unwrap()) as usize;
        let bucket = i32::from_le_bytes(args[48..52].try_into().unwrap()) as usize;
        let minn   = i32::from_le_bytes(args[52..56].try_into().unwrap()) as usize;
        let maxn   = i32::from_le_bytes(args[56..60].try_into().unwrap()) as usize;

        // --- dictionary ---
        let dict_size     = read_i32(&mut r)? as usize;
        let nwords        = read_i32(&mut r)? as usize;
        let _nlabels      = read_i32(&mut r)?;
        let _ntokens      = read_i64(&mut r)?;
        let pruneidx_size = read_i64(&mut r)? as usize;

        let mut vocab = Vec::with_capacity(nwords);
        let mut index = HashMap::with_capacity(nwords);

        for dict_pos in 0..dict_size {
            let word       = read_cstring(&mut r)?;
            let _count     = read_i64(&mut r)?;
            let entry_type = read_i8(&mut r)?;
            if entry_type == 0 {
                index.insert(word.clone(), dict_pos);
                vocab.push(word);
            }
        }
        // Skip pruning index (pruneidx_size pairs of i32)
        if pruneidx_size > 0 {
            let mut discard = vec![0u8; pruneidx_size * 8];
            r.read_exact(&mut discard)?;
        }

        // --- quantized flag ---
        let quantized = read_i8(&mut r)?;
        if quantized != 0 {
            return Err(Error::Parse(
                "quantized FastText models are not yet supported".into(),
            ));
        }

        // --- input matrix ---
        let matrix_rows = read_i64(&mut r)? as usize;
        let matrix_cols = read_i64(&mut r)? as usize;
        if matrix_cols != dim {
            return Err(Error::Parse(format!(
                "matrix column count {matrix_cols} does not match dim {dim}"
            )));
        }
        let n_floats = matrix_rows * matrix_cols;
        let mut raw_bytes = vec![0u8; n_floats * 4];
        r.read_exact(&mut raw_bytes)?;
        let input_matrix: Vec<f32> = raw_bytes
            .chunks_exact(4)
            .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
            .collect();

        // Pre-compute L2 norms for word rows to speed up most_similar.
        let word_norms: Vec<f32> = (0..nwords)
            .map(|i| {
                let row = &input_matrix[i * dim..(i + 1) * dim];
                row.iter().map(|x| x * x).sum::<f32>().sqrt()
            })
            .collect();

        Ok(Self { vocab, index, input_matrix, word_norms, nwords, bucket, dim, minn, maxn })
    }

    /// Returns all vocabulary words in load order.
    pub fn get_vocabs(&self) -> &[String] {
        &self.vocab
    }

    #[inline]
    fn row(&self, idx: usize) -> &[f32] {
        &self.input_matrix[idx * self.dim..(idx + 1) * self.dim]
    }

    /// Returns subword bucket indices for `word` (byte n-grams of `<word>`).
    fn subword_indices(&self, word: &str) -> Vec<usize> {
        if self.minn == 0 || self.maxn == 0 || self.bucket == 0 {
            return vec![];
        }
        let bounded = format!("<{}>", word);
        let bytes = bounded.as_bytes();
        let len = bytes.len();
        let mut out = Vec::new();
        for i in 0..len {
            for j in (i + self.minn)..=(i + self.maxn).min(len) {
                let h = fasttext_hash(&bytes[i..j]) as usize % self.bucket;
                out.push(self.nwords + h);
            }
        }
        out
    }

    /// Averages word + subword rows, then L2-normalises.
    ///
    /// Returns `None` only when the word is fully OOV *and* there are no
    /// subword buckets configured (minn == 0 || maxn == 0).
    fn compute_vector(&self, word: &str) -> Option<Vec<f32>> {
        let total_rows = self.input_matrix.len() / self.dim;
        let mut v = vec![0.0f32; self.dim];
        let mut count = 0usize;

        if let Some(&idx) = self.index.get(word) {
            let row = self.row(idx);
            for (a, b) in v.iter_mut().zip(row) { *a += b; }
            count += 1;
        }

        for sub_idx in self.subword_indices(word) {
            if sub_idx < total_rows {
                let row = self.row(sub_idx);
                for (a, b) in v.iter_mut().zip(row) { *a += b; }
                count += 1;
            }
        }

        if count == 0 { return None; }
        let n = count as f32;
        for x in &mut v { *x /= n; }
        l2_normalize(&mut v);
        Some(v)
    }
}

impl WordEmbedding for FastTextEmbedding {
    fn dim(&self) -> usize { self.dim }

    fn embed_word(&self, word: &str) -> Option<Vec<f32>> {
        self.compute_vector(word)
    }

    /// Returns `true` if `word` is in-vocabulary OR can be represented via subwords.
    fn contains(&self, word: &str) -> bool {
        self.index.contains_key(word)
            || (!self.subword_indices(word).is_empty())
    }

    /// Returns the `n` most similar words by cosine similarity.
    ///
    /// Uses pre-normalised norms against raw word-row dot products (no
    /// subword averaging for candidates) — O(vocab × dim).
    fn most_similar(&self, word: &str, n: usize) -> Vec<(String, f32)> {
        let query = match self.compute_vector(word) {
            Some(v) => v,
            None => return vec![],
        };
        let exclude_idx = self.index.get(word).copied();

        let mut scores: Vec<(usize, f32)> = (0..self.vocab.len())
            .filter(|&i| Some(i) != exclude_idx)
            .map(|vocab_i| {
                // vocab index == dict_pos == row index for skipgram/cbow models
                let dict_pos = self.index[&self.vocab[vocab_i]];
                let row = self.row(dict_pos);
                let norm = self.word_norms[dict_pos];
                let score = if norm > 1e-12 { dot(&query, row) / norm } else { 0.0 };
                (vocab_i, score)
            })
            .collect();

        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scores.truncate(n);
        scores.into_iter().map(|(i, s)| (self.vocab[i].clone(), s)).collect()
    }

    fn load(path: &Path) -> Result<Self> {
        Self::from_file(path)
    }
}

// ---------------------------------------------------------------------------
// Sentence embedding convenience alias
// ---------------------------------------------------------------------------

/// Sentence embedding backed by a FastText model.
///
/// Averages the subword-aware FastText word vectors for each token produced by
/// [`word_tokenize`](crate::word_tokenizer::word_tokenize), giving the same
/// result as averaging word vectors from a FastText binary model
/// for each token in the sentence.
///
/// # Example
///
/// ```no_run
/// use std::path::Path;
/// use rustam::fasttext::FastTextSentEmbedding;
/// use rustam::embedding::SentenceEmbedding;
///
/// let model = FastTextSentEmbedding::load(Path::new("fasttext_skipgram_300.bin")).unwrap();
/// let sim = model.similarity_str("من کتاب می‌خوانم", "او کتاب خواند").unwrap();
/// ```
pub type FastTextSentEmbedding = crate::word2vec::AvgSentEmbedding<FastTextEmbedding>;

// ---------------------------------------------------------------------------
// Binary I/O helpers
// ---------------------------------------------------------------------------

fn read_u32<R: Read>(r: &mut R) -> Result<u32> {
    let mut b = [0u8; 4];
    r.read_exact(&mut b)?;
    Ok(u32::from_le_bytes(b))
}

fn read_i32<R: Read>(r: &mut R) -> Result<i32> {
    let mut b = [0u8; 4];
    r.read_exact(&mut b)?;
    Ok(i32::from_le_bytes(b))
}

fn read_i64<R: Read>(r: &mut R) -> Result<i64> {
    let mut b = [0u8; 8];
    r.read_exact(&mut b)?;
    Ok(i64::from_le_bytes(b))
}

fn read_i8<R: Read>(r: &mut R) -> Result<i8> {
    let mut b = [0u8; 1];
    r.read_exact(&mut b)?;
    Ok(b[0] as i8)
}

fn read_cstring<R: Read>(r: &mut R) -> Result<String> {
    let mut bytes = Vec::new();
    let mut b = [0u8; 1];
    loop {
        r.read_exact(&mut b)?;
        if b[0] == 0 { break; }
        bytes.push(b[0]);
    }
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

/// FNV-1a hash matching FastText C++ `Dictionary::hash`.
///
/// The C++ code casts each byte to `int8_t` before XOR, which sign-extends
/// bytes > 127.  We replicate that here so bucket indices are identical.
fn fasttext_hash(bytes: &[u8]) -> u32 {
    let mut h: u32 = 2166136261;
    for &b in bytes {
        let signed = (b as i8 as i32) as u32; // sign-extend, then zero-extend
        h ^= signed;
        h = h.wrapping_mul(16777619);
    }
    h
}

fn l2_normalize(v: &mut [f32]) {
    let norm: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 1e-12 {
        for x in v.iter_mut() { *x /= norm; }
    }
}

fn dot(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Writes a minimal FastText `.bin` file in memory and reads it back.
    fn make_fasttext_bin(words: &[(&str, [f32; 3])], bucket: i32, minn: i32, maxn: i32) -> Vec<u8> {
        let mut out = Vec::new();
        let dim: i32 = 3;

        // magic + version
        out.extend_from_slice(&MAGIC.to_le_bytes());
        out.extend_from_slice(&VERSION.to_le_bytes());

        // args (72 bytes): lr(f64) lrUpdateRate(i32) dim(i32) ws(i32) epoch(i32)
        //   minCount(i32) minCountLabel(i32) neg(i32) wordNgrams(i32) loss(i32)
        //   model(i32) bucket(i32) minn(i32) maxn(i32) lrUpdateRate2(i32) t(f64)
        out.extend_from_slice(&0.05f64.to_le_bytes()); // lr
        out.extend_from_slice(&100i32.to_le_bytes());  // lrUpdateRate
        out.extend_from_slice(&dim.to_le_bytes());     // dim [offset 12]
        out.extend_from_slice(&5i32.to_le_bytes());    // ws
        out.extend_from_slice(&5i32.to_le_bytes());    // epoch
        out.extend_from_slice(&5i32.to_le_bytes());    // minCount
        out.extend_from_slice(&0i32.to_le_bytes());    // minCountLabel
        out.extend_from_slice(&5i32.to_le_bytes());    // neg
        out.extend_from_slice(&1i32.to_le_bytes());    // wordNgrams
        out.extend_from_slice(&1i32.to_le_bytes());    // loss (ns)
        out.extend_from_slice(&2i32.to_le_bytes());    // model (skipgram)
        out.extend_from_slice(&bucket.to_le_bytes());  // bucket [offset 48]
        out.extend_from_slice(&minn.to_le_bytes());    // minn [offset 52]
        out.extend_from_slice(&maxn.to_le_bytes());    // maxn [offset 56]
        out.extend_from_slice(&100i32.to_le_bytes());  // lrUpdateRate (dup)
        out.extend_from_slice(&1e-4f64.to_le_bytes()); // t
        assert_eq!(out.len(), 8 + ARGS_NBYTES); // 8 = magic+version

        // dictionary header
        let nwords = words.len() as i32;
        out.extend_from_slice(&nwords.to_le_bytes()); // size
        out.extend_from_slice(&nwords.to_le_bytes()); // nwords
        out.extend_from_slice(&0i32.to_le_bytes());   // nlabels
        out.extend_from_slice(&(nwords as i64 * 1000).to_le_bytes()); // ntokens
        out.extend_from_slice(&0i64.to_le_bytes());   // pruneidx_size

        // dictionary entries
        for (word, _) in words {
            out.extend_from_slice(word.as_bytes());
            out.push(0u8); // null terminator
            out.extend_from_slice(&1000i64.to_le_bytes()); // count
            out.push(0u8); // type = word
        }

        // quantized = false
        out.push(0u8);

        // input matrix: rows = nwords + bucket, cols = dim
        let total_rows = (nwords as i64) + (bucket as i64);
        out.extend_from_slice(&total_rows.to_le_bytes());
        out.extend_from_slice(&(dim as i64).to_le_bytes());
        for (_, vec) in words {
            for &f in vec { out.extend_from_slice(&f.to_le_bytes()); }
        }
        // fill bucket rows with zeros
        let zero_f: f32 = 0.0;
        for _ in 0..(bucket as usize * dim as usize) {
            out.extend_from_slice(&zero_f.to_le_bytes());
        }

        out
    }

    fn load_from_bytes(bytes: Vec<u8>, tag: &str) -> FastTextEmbedding {
        let path = std::env::temp_dir().join(format!("rustam_ft_{tag}.bin"));
        std::fs::write(&path, bytes).unwrap();
        FastTextEmbedding::from_file(&path).unwrap()
    }

    #[test]
    fn loads_word_vectors() {
        let words = [("کتاب", [1.0, 0.0, 0.0]), ("دفتر", [0.9, 0.1, 0.0])];
        let m = load_from_bytes(make_fasttext_bin(&words, 100, 3, 6), "load");
        assert!(m.embed_word("کتاب").is_some());
        assert_eq!(m.vocab.len(), 2);
        assert_eq!(m.dim, 3);
    }

    #[test]
    fn embed_word_normalised() {
        let words = [("ایران", [3.0, 4.0, 0.0])]; // norm=5 → normalised to unit length
        let m = load_from_bytes(make_fasttext_bin(&words, 0, 0, 0), "norm");
        let v = m.embed_word("ایران").unwrap();
        let norm: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 1e-5);
    }

    #[test]
    fn oov_with_subwords() {
        let words = [("کتاب", [1.0, 0.0, 0.0])];
        let m = load_from_bytes(make_fasttext_bin(&words, 100, 1, 3), "oov_sub");
        // "کتابخانه" is OOV but can use character n-gram buckets
        assert!(m.embed_word("کتابخانه").is_some());
    }

    #[test]
    fn oov_no_subwords_returns_none() {
        let words = [("کتاب", [1.0, 0.0, 0.0])];
        let m = load_from_bytes(make_fasttext_bin(&words, 0, 0, 0), "oov_nosub");
        assert!(m.embed_word("xyz").is_none());
    }

    #[test]
    fn most_similar_returns_closest() {
        let words = [
            ("کتاب",   [1.0f32, 0.0, 0.0]),
            ("دفتر",   [0.99,   0.1, 0.0]),
            ("پنجره",  [0.0,    0.0, 1.0]),
        ];
        let m = load_from_bytes(make_fasttext_bin(&words, 0, 0, 0), "sim");
        let sim = m.most_similar("کتاب", 1);
        assert_eq!(sim.len(), 1);
        assert_eq!(sim[0].0, "دفتر");
    }

    #[test]
    fn bad_magic_returns_error() {
        let mut bytes = make_fasttext_bin(&[("a", [1.0, 0.0, 0.0])], 0, 0, 0);
        bytes[0] = 0xFF; // corrupt magic
        let path = std::env::temp_dir().join("rustam_ft_bad_magic.bin");
        std::fs::write(&path, &bytes).unwrap();
        assert!(FastTextEmbedding::from_file(&path).is_err());
    }
}

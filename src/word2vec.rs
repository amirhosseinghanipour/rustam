//! Concrete word and sentence embedding backends.
//!
//! [`Word2VecEmbedding`] reads models in word2vec binary (`.bin`) or text
//! (`.vec` / `.txt`) format, supporting both `"keyedvector"` and
//! GloVe-converted-to-word2vec model files.
//!
//! [`AvgSentEmbedding`] wraps any [`WordEmbedding`] and implements
//! [`SentenceEmbedding`] by averaging word vectors.  It is a simple but
//! effective baseline for sentence similarity tasks.
//!
//! # Examples
//!
//! ```no_run
//! use std::path::Path;
//! use rustam::word2vec::Word2VecEmbedding;
//! use rustam::embedding::WordEmbedding;
//!
//! // Auto-detects binary vs text format.
//! let model = Word2VecEmbedding::from_file(Path::new("vectors.bin")).unwrap();
//!
//! let vec = model.embed_word("کتاب");
//! let similar = model.most_similar("ایران", 5);
//! let odd = model.doesnt_match(&["کتاب", "دفتر", "قلم", "پنجره"]);
//! ```

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

use crate::embedding::{SentenceEmbedding, WordEmbedding};
use crate::error::{Error, Result};

// ---------------------------------------------------------------------------
// Word2VecEmbedding
// ---------------------------------------------------------------------------

/// Word embeddings loaded from a word2vec binary or text model file.
///
/// All vectors are L2-normalised at load time so that cosine similarity
/// reduces to a plain dot product and `most_similar` stays fast.
///
/// Supports word2vec binary and text formats, as well as
/// GloVe-converted-to-word2vec text files.
pub struct Word2VecEmbedding {
    vocab: Vec<String>,
    index: HashMap<String, usize>,
    /// Row-major matrix: `dim` L2-normalised floats per word.
    vectors: Vec<f32>,
    dim: usize,
}

impl Word2VecEmbedding {
    /// Loads a model, auto-detecting binary vs text format.
    ///
    /// Tries binary first; falls back to text on failure.
    pub fn from_file(path: &Path) -> Result<Self> {
        Self::from_binary(path).or_else(|_| Self::from_text(path))
    }

    /// Loads a word2vec **binary** format model (`.bin`).
    ///
    /// Format: header line `"{num_words} {num_dims}\n"`, then for each word
    /// the UTF-8 word bytes (leading newlines are skipped), a space, then
    /// `num_dims × 4` bytes of little-endian `f32`.
    pub fn from_binary(path: &Path) -> Result<Self> {
        let f = std::fs::File::open(path)?;
        let mut reader = BufReader::new(f);

        let mut header = String::new();
        reader.read_line(&mut header)?;
        let (num_words, dim) = parse_header(&header)?;

        let mut vocab = Vec::with_capacity(num_words);
        let mut vectors = Vec::with_capacity(num_words * dim);
        let mut byte_buf = [0u8; 1];

        for _ in 0..num_words {
            // Read word bytes; skip leading newlines, stop at space.
            let mut word_bytes = Vec::new();
            loop {
                reader.read_exact(&mut byte_buf)?;
                match byte_buf[0] {
                    b' ' => break,
                    b'\n' if word_bytes.is_empty() => {} // skip leading newlines
                    b => word_bytes.push(b),
                }
            }
            let word = String::from_utf8_lossy(&word_bytes).into_owned();

            // Read the vector (dim × f32).
            let mut raw = vec![0u8; dim * 4];
            reader.read_exact(&mut raw)?;
            let mut row: Vec<f32> = raw
                .chunks_exact(4)
                .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
                .collect();

            l2_normalize(&mut row);
            vocab.push(word);
            vectors.extend_from_slice(&row);
        }

        Ok(Self::build(vocab, vectors, dim))
    }

    /// Loads a word2vec **text** format model (`.vec` / `.txt`).
    ///
    /// Format: header line `"{num_words} {num_dims}"`, then one line per word:
    /// `"{word} {f1} {f2} … {fdim}"`.
    pub fn from_text(path: &Path) -> Result<Self> {
        let f = std::fs::File::open(path)?;
        let reader = BufReader::new(f);
        let mut lines = reader.lines();

        let header = lines
            .next()
            .ok_or_else(|| Error::Parse("embedding file is empty".to_string()))??;
        let (num_words, dim) = parse_header(&header)?;

        let mut vocab = Vec::with_capacity(num_words);
        let mut vectors = Vec::with_capacity(num_words * dim);

        for line in lines {
            let line = line?;
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // Split at most dim+1 parts (word + dim values).
            let mut parts = line.splitn(dim + 2, ' ');
            let word = match parts.next() {
                Some(w) => w.to_string(),
                None => continue,
            };

            let mut row = Vec::with_capacity(dim);
            for val in parts {
                if let Ok(v) = val.parse::<f32>() {
                    row.push(v);
                }
            }
            if row.len() < dim {
                continue; // skip malformed lines
            }
            row.truncate(dim);

            l2_normalize(&mut row);
            vocab.push(word);
            vectors.extend_from_slice(&row);
        }

        Ok(Self::build(vocab, vectors, dim))
    }

    /// Returns the vocabulary in load order.
    pub fn get_vocabs(&self) -> &[String] {
        &self.vocab
    }

    fn build(vocab: Vec<String>, vectors: Vec<f32>, dim: usize) -> Self {
        let index: HashMap<String, usize> =
            vocab.iter().enumerate().map(|(i, w)| (w.clone(), i)).collect();
        Self { vocab, index, vectors, dim }
    }

    #[inline]
    fn row(&self, idx: usize) -> &[f32] {
        &self.vectors[idx * self.dim..(idx + 1) * self.dim]
    }
}

impl WordEmbedding for Word2VecEmbedding {
    fn dim(&self) -> usize {
        self.dim
    }

    fn embed_word(&self, word: &str) -> Option<Vec<f32>> {
        let &idx = self.index.get(word)?;
        Some(self.row(idx).to_vec())
    }

    /// Returns the `n` most similar words to `word` by cosine similarity.
    ///
    /// O(vocab × dim) — acceptable for typical 50K–200K vocabulary sizes.
    fn most_similar(&self, word: &str, n: usize) -> Vec<(String, f32)> {
        let &query_idx = match self.index.get(word) {
            Some(i) => i,
            None => return vec![],
        };
        let query = self.row(query_idx);

        let mut scores: Vec<(usize, f32)> = (0..self.vocab.len())
            .filter(|&i| i != query_idx)
            .map(|i| (i, dot(query, self.row(i))))
            .collect();

        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scores.truncate(n);
        scores
            .into_iter()
            .map(|(i, score)| (self.vocab[i].clone(), score))
            .collect()
    }

    fn load(path: &Path) -> Result<Self> {
        Self::from_file(path)
    }
}

// ---------------------------------------------------------------------------
// AvgSentEmbedding
// ---------------------------------------------------------------------------

/// Sentence embedding that averages constituent word vectors.
///
/// A simple but effective baseline: the sentence vector is the mean of the
/// L2-normalised word vectors of all in-vocabulary tokens.  Out-of-vocabulary
/// tokens are silently skipped.  If *all* tokens are OOV the zero vector is
/// returned.
///
/// ```no_run
/// use std::path::Path;
/// use rustam::word2vec::{AvgSentEmbedding, Word2VecEmbedding};
/// use rustam::embedding::SentenceEmbedding;
///
/// let words = Word2VecEmbedding::from_file(Path::new("vectors.bin")).unwrap();
/// let model = AvgSentEmbedding::new(words);
/// let vec = model.embed(&["من", "کتاب", "می‌خوانم"]).unwrap();
/// ```
pub struct AvgSentEmbedding<W: WordEmbedding> {
    word_model: W,
}

impl<W: WordEmbedding> AvgSentEmbedding<W> {
    /// Wraps a word embedding model.
    pub fn new(word_model: W) -> Self {
        Self { word_model }
    }

    /// Returns a reference to the underlying word embedding model.
    pub fn word_model(&self) -> &W {
        &self.word_model
    }
}

impl<W: WordEmbedding> SentenceEmbedding for AvgSentEmbedding<W> {
    fn dim(&self) -> usize {
        self.word_model.dim()
    }

    fn embed(&self, tokens: &[&str]) -> Result<Vec<f32>> {
        let mut avg = vec![0.0f32; self.dim()];
        let mut count = 0usize;
        for &tok in tokens {
            if let Some(v) = self.word_model.embed_word(tok) {
                for (a, b) in avg.iter_mut().zip(v.iter()) {
                    *a += b;
                }
                count += 1;
            }
        }
        if count > 0 {
            let n = count as f32;
            for a in &mut avg {
                *a /= n;
            }
        }
        Ok(avg)
    }

    /// Loads the underlying word model from `path` and wraps it.
    ///
    /// Equivalent to `AvgSentEmbedding::new(W::load(path)?)`.
    fn load(path: &Path) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self::new(W::load(path)?))
    }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn parse_header(header: &str) -> Result<(usize, usize)> {
    let mut parts = header.trim().split_whitespace();
    let nw = parts
        .next()
        .ok_or_else(|| Error::Parse("missing num_words in header".to_string()))?
        .parse::<usize>()
        .map_err(|e| Error::Parse(format!("invalid num_words: {e}")))?;
    let nd = parts
        .next()
        .ok_or_else(|| Error::Parse("missing num_dims in header".to_string()))?
        .parse::<usize>()
        .map_err(|e| Error::Parse(format!("invalid num_dims: {e}")))?;
    Ok((nw, nd))
}

fn l2_normalize(v: &mut [f32]) {
    let norm: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 1e-12 {
        for x in v.iter_mut() {
            *x /= norm;
        }
    }
}

fn dot(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::embedding::WordEmbedding;

    fn make_model() -> Word2VecEmbedding {
        // Minimal in-memory text model with 3 words and dim=3.
        // Using a temp file so we exercise the actual parser.
        let content = "3 3\nکتاب 1.0 0.0 0.0\nدفتر 0.9 0.1 0.0\nپنجره 0.0 0.0 1.0\n";
        let dir = std::env::temp_dir();
        let path = dir.join("rustam_test_w2v.vec");
        std::fs::write(&path, content).unwrap();
        Word2VecEmbedding::from_text(&path).unwrap()
    }

    #[test]
    fn embed_word_known() {
        let m = make_model();
        assert!(m.embed_word("کتاب").is_some());
    }

    #[test]
    fn embed_word_oov() {
        let m = make_model();
        assert!(m.embed_word("xyz").is_none());
    }

    #[test]
    fn most_similar_top1() {
        let m = make_model();
        let sim = m.most_similar("کتاب", 1);
        assert_eq!(sim.len(), 1);
        assert_eq!(sim[0].0, "دفتر");
    }

    #[test]
    fn doesnt_match_odd_word() {
        let m = make_model();
        let odd = m.doesnt_match(&["کتاب", "دفتر", "پنجره"]);
        assert_eq!(odd, Some("پنجره".to_string()));
    }

    #[test]
    fn avg_sent_embedding() {
        use super::AvgSentEmbedding;
        use crate::embedding::SentenceEmbedding;
        let m = make_model();
        let sent = AvgSentEmbedding::new(m);
        let v = sent.embed(&["کتاب", "دفتر"]).unwrap();
        assert_eq!(v.len(), 3);
        assert!(v[0] > 0.0); // both words have positive x component
    }

    #[test]
    fn avg_sent_all_oov() {
        use super::AvgSentEmbedding;
        use crate::embedding::SentenceEmbedding;
        let m = make_model();
        let sent = AvgSentEmbedding::new(m);
        let v = sent.embed(&["xyz", "abc"]).unwrap();
        assert!(v.iter().all(|&x| x == 0.0)); // all OOV → zero vector
    }
}

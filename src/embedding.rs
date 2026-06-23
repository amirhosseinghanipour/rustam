//! Word and sentence embedding traits for Persian NLP.
//!
//! These traits define the interface that any embedding backend must implement.
//! Concrete backends (FastText, Word2Vec, transformer encoders) can implement
//! these traits independently and plug into downstream components that accept
//! `impl WordEmbedding` or `impl SentenceEmbedding`.

use crate::error::Result;

/// A model that maps words to fixed-length dense vectors.
pub trait WordEmbedding {
    /// Returns the embedding dimension.
    fn dim(&self) -> usize;

    /// Returns the embedding vector for `word`, or `None` if out-of-vocabulary.
    fn embed_word(&self, word: &str) -> Option<Vec<f32>>;

    /// Returns `true` if `word` has a representation in this model.
    fn contains(&self, word: &str) -> bool {
        self.embed_word(word).is_some()
    }

    /// Returns the embedding for `word` or the zero vector if out-of-vocabulary.
    fn embed_word_or_zero(&self, word: &str) -> Vec<f32> {
        self.embed_word(word)
            .unwrap_or_else(|| vec![0.0; self.dim()])
    }

    /// Computes the cosine similarity between the embeddings of two words.
    ///
    /// Returns `None` if either word is out-of-vocabulary.
    fn similarity(&self, a: &str, b: &str) -> Option<f32> {
        let va = self.embed_word(a)?;
        let vb = self.embed_word(b)?;
        Some(cosine(&va, &vb))
    }

    /// Returns the `n` words most similar to `word` (excluding `word` itself).
    ///
    /// Default implementation is O(vocab × dim) — backends should override
    /// this with an index-based nearest-neighbour search if performance matters.
    fn most_similar(&self, word: &str, n: usize) -> Vec<(String, f32)>;

    /// Returns the word from `words` that is most dissimilar to the others.
    ///
    /// Uses average pairwise cosine similarity: the word with the lowest mean
    /// similarity to all other words is the odd one out.  Returns `None` if
    /// any word is out-of-vocabulary.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rustam::embedding::WordEmbedding;
    /// # fn example(model: &impl WordEmbedding) {
    /// let odd = model.doesnt_match(&["کتاب", "دفتر", "قلم", "پنجره"]);
    /// assert_eq!(odd, Some("پنجره".to_string()));
    /// # }
    /// ```
    fn doesnt_match(&self, words: &[&str]) -> Option<String> {
        if words.is_empty() {
            return None;
        }
        // Need embeddings for all words; bail out if any is OOV.
        let vecs: Vec<Vec<f32>> = words.iter()
            .map(|&w| self.embed_word(w))
            .collect::<Option<Vec<_>>>()?;

        let n = words.len();
        let odd_idx = (0..n).min_by(|&a, &b| {
            let avg = |i: usize| -> f32 {
                if n <= 1 { return 0.0; }
                let sum: f32 = (0..n)
                    .filter(|&j| j != i)
                    .map(|j| cosine(&vecs[i], &vecs[j]))
                    .sum();
                sum / (n - 1) as f32
            };
            avg(a).partial_cmp(&avg(b)).unwrap_or(std::cmp::Ordering::Equal)
        })?;
        Some(words[odd_idx].to_string())
    }

    /// Returns the number of dimensions in the embedding space.
    fn get_vector_size(&self) -> usize {
        self.dim()
    }

    /// Loads the model from `path`.
    fn load(path: &std::path::Path) -> Result<Self>
    where
        Self: Sized;
}

/// A model that maps token sequences (sentences) to fixed-length dense vectors.
pub trait SentenceEmbedding {
    /// Returns the embedding dimension.
    fn dim(&self) -> usize;

    /// Encodes a sequence of tokens into a single vector.
    fn embed(&self, tokens: &[&str]) -> Result<Vec<f32>>;

    /// Encodes a raw string, normalising and tokenising with `WordTokenizer`.
    fn embed_str(&self, text: &str) -> Result<Vec<f32>> {
        let owned = crate::word_tokenizer::word_tokenize(text);
        let tokens: Vec<&str> = owned.iter().map(|s| s.as_str()).collect();
        self.embed(&tokens)
    }

    /// Computes cosine similarity between the embeddings of two raw strings.
    ///
    /// Each string is tokenised with [`word_tokenize`](crate::word_tokenizer::word_tokenize)
    /// before encoding.
    fn similarity_str(&self, a: &str, b: &str) -> Result<f32> {
        let va = self.embed_str(a)?;
        let vb = self.embed_str(b)?;
        Ok(cosine(&va, &vb))
    }

    /// Computes the cosine similarity between the encodings of two token sequences.
    fn similarity(&self, a: &[&str], b: &[&str]) -> Result<f32> {
        let va = self.embed(a)?;
        let vb = self.embed(b)?;
        Ok(cosine(&va, &vb))
    }

    /// Loads the model from `path`.
    fn load(path: &std::path::Path) -> Result<Self>
    where
        Self: Sized;
}

// ---------------------------------------------------------------------------
// Internal math helpers
// ---------------------------------------------------------------------------

fn cosine(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let na: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let nb: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if na == 0.0 || nb == 0.0 {
        0.0
    } else {
        dot / (na * nb)
    }
}

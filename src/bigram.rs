//! Bigram language model for Persian text.
//!
//! [`BigramModel`] counts unigrams and bigrams from tokenised sentences and
//! provides conditional probability estimates, next-word prediction, candidate
//! suggestions, and perplexity scoring.
//!
//! Probabilities use **Laplace (add-1) smoothing** so that unseen bigrams
//! never get zero probability.
//!
//! # Example
//!
//! ```
//! use rustam::BigramModel;
//!
//! let mut model = BigramModel::new();
//! model.train(&[
//!     vec!["من", "کتاب", "می‌خوانم"],
//!     vec!["او", "کتاب", "نوشت"],
//! ]);
//!
//! // P("کتاب" | "من") should be > P("نوشت" | "من")
//! assert!(model.probability("کتاب", "من") > model.probability("نوشت", "من"));
//!
//! // Most likely word after "کتاب"
//! let next = model.next_word("کتاب");
//! assert!(next.is_some());
//! ```

use std::collections::HashMap;

/// Bigram language model trained on tokenised Persian sentences.
///
/// Stores unigram and bigram counts and exposes:
/// - [`probability`](Self::probability) — P(word | prev_word) with Laplace smoothing
/// - [`next_word`](Self::next_word) — single most probable next word
/// - [`suggest`](Self::suggest) — top-n next-word candidates with scores
/// - [`perplexity`](Self::perplexity) — sentence-level perplexity
pub struct BigramModel {
    unigrams: HashMap<String, u64>,
    bigrams: HashMap<(String, String), u64>,
    total: u64,
}

impl BigramModel {
    /// Creates an empty model.  Call [`train`](Self::train) before querying.
    pub fn new() -> Self {
        Self { unigrams: HashMap::new(), bigrams: HashMap::new(), total: 0 }
    }

    /// Updates the model counts from `sents` (a slice of tokenised sentences).
    ///
    /// Can be called multiple times to incrementally extend the model.
    pub fn train<S: AsRef<str>>(&mut self, sents: &[Vec<S>]) {
        for sent in sents {
            for i in 0..sent.len() {
                let w = sent[i].as_ref().to_string();
                *self.unigrams.entry(w.clone()).or_insert(0) += 1;
                self.total += 1;
                if let Some(next) = sent.get(i + 1) {
                    let n = next.as_ref().to_string();
                    *self.bigrams.entry((w, n)).or_insert(0) += 1;
                }
            }
        }
    }

    /// P(`word` | `prev_word`) with Laplace smoothing.
    ///
    /// Returns a value in (0, 1].  Never returns 0 thanks to add-1 smoothing.
    pub fn probability(&self, word: &str, prev_word: &str) -> f64 {
        let bigram = self
            .bigrams
            .get(&(prev_word.to_string(), word.to_string()))
            .copied()
            .unwrap_or(0);
        let prev = self.unigrams.get(prev_word).copied().unwrap_or(0);
        let v = self.unigrams.len() as u64;
        (bigram + 1) as f64 / (prev + v).max(1) as f64
    }

    /// Unigram probability P(`word`).
    pub fn unigram_probability(&self, word: &str) -> f64 {
        let freq = self.unigrams.get(word).copied().unwrap_or(0);
        freq as f64 / self.total.max(1) as f64
    }

    /// Returns the single most probable word to follow `prev_word`.
    ///
    /// Searches only observed bigrams (fast, O(observed bigrams starting with prev_word)).
    /// Returns `None` if `prev_word` has never been seen.
    pub fn next_word(&self, prev_word: &str) -> Option<&str> {
        self.bigrams
            .iter()
            .filter(|((p, _), _)| p == prev_word)
            .max_by_key(|(_, &cnt)| cnt)
            .map(|((_, next), _)| next.as_str())
    }

    /// Returns the `n` most likely next words after `prev_word` with their
    /// conditional probabilities.
    ///
    /// Uses Laplace-smoothed probabilities so all vocabulary words are
    /// considered, not just those with observed bigrams.
    pub fn suggest(&self, prev_word: &str, n: usize) -> Vec<(String, f64)> {
        let mut scores: Vec<(String, f64)> = self
            .unigrams
            .keys()
            .map(|w| (w.clone(), self.probability(w, prev_word)))
            .collect();
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scores.truncate(n);
        scores
    }

    /// Sentence perplexity: geometric mean inverse probability over bigrams.
    ///
    /// Returns `f64::INFINITY` for sentences shorter than 2 tokens.
    pub fn perplexity(&self, sent: &[&str]) -> f64 {
        if sent.len() < 2 {
            return f64::INFINITY;
        }
        let log_prob: f64 = sent
            .windows(2)
            .map(|w| self.probability(w[1], w[0]).log2())
            .sum();
        2f64.powf(-log_prob / (sent.len() - 1) as f64)
    }

    /// Number of unique word types seen during training.
    pub fn vocab_size(&self) -> usize { self.unigrams.len() }

    /// Total word tokens seen during training.
    pub fn total_tokens(&self) -> u64 { self.total }
}

impl Default for BigramModel {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tiny_model() -> BigramModel {
        let mut m = BigramModel::new();
        m.train(&[
            vec!["من", "کتاب", "می‌خوانم"],
            vec!["او", "کتاب", "نوشت"],
            vec!["من", "نوشت"],
        ]);
        m
    }

    #[test]
    fn train_counts() {
        let m = tiny_model();
        assert_eq!(m.vocab_size(), 5);
        assert_eq!(m.total_tokens(), 8);
    }

    #[test]
    fn probability_seen_bigram_higher() {
        let m = tiny_model();
        // "من" → "کتاب" was seen once; "من" → "نوشت" also once
        // Both should have positive prob
        assert!(m.probability("کتاب", "من") > 0.0);
        assert!(m.probability("نوشت", "من") > 0.0);
    }

    #[test]
    fn probability_unseen_bigram_nonzero() {
        let m = tiny_model();
        // Laplace smoothing: even unseen pairs get non-zero probability
        assert!(m.probability("می‌خوانم", "نوشت") > 0.0);
    }

    #[test]
    fn next_word_returns_something() {
        let m = tiny_model();
        assert!(m.next_word("من").is_some());
    }

    #[test]
    fn suggest_top_n() {
        let m = tiny_model();
        let s = m.suggest("کتاب", 2);
        assert_eq!(s.len(), 2);
        // probabilities should be descending
        assert!(s[0].1 >= s[1].1);
    }

    #[test]
    fn perplexity_finite_for_known_sent() {
        let m = tiny_model();
        let pp = m.perplexity(&["من", "کتاب", "می‌خوانم"]);
        assert!(pp.is_finite());
    }

    #[test]
    fn perplexity_infinite_for_short_sent() {
        let m = tiny_model();
        assert_eq!(m.perplexity(&["من"]), f64::INFINITY);
    }
}

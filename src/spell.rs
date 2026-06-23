//! Persian spell corrector.
//!
//! [`SpellCorrector`] uses the embedded word-frequency lexicon and character-level
//! edit operations to suggest the most probable correction for a misspelled Persian
//! word using the Peter Norvig algorithm.
//!
//! The algorithm:
//! 1. If the word is known, return it unchanged.
//! 2. Try all single-edit candidates; return known ones ranked by corpus frequency.
//! 3. Try all two-edit candidates; return known ones ranked by corpus frequency.
//! 4. Return the original word unchanged.
//!
//! # Example
//!
//! ```
//! use rustam::SpellCorrector;
//!
//! let spell = SpellCorrector::new();
//! // "کتاب" is correctly spelled — should come back unchanged
//! assert_eq!(spell.correction("کتاب"), "کتاب");
//! // unknown words fall through to the original
//! let _ = spell.correction("xyz123");
//! ```

use std::collections::{HashMap, HashSet};

use crate::data::{parse_words, WORDS_DAT};

/// Persian letters used when generating edit candidates.
///
/// Includes common Arabic-script variants that appear in Persian text
/// (Arabic ي vs Persian ی, Arabic ك vs Persian ک, etc.) so that the
/// corrector handles the most frequent spelling confusion classes.
const LETTERS: &[char] = &[
    'ا', 'آ', 'أ', 'إ',
    'ب', 'پ', 'ت', 'ث',
    'ج', 'چ', 'ح', 'خ',
    'د', 'ذ', 'ر', 'ز', 'ژ',
    'س', 'ش', 'ص', 'ض',
    'ط', 'ظ', 'ع', 'غ',
    'ف', 'ق', 'ک', 'ك', 'گ',
    'ل', 'م', 'ن',
    'و', 'ؤ',
    'ه', 'ة', 'ۀ',
    'ی', 'ي', 'ى', 'ئ',
];

/// Persian spell corrector backed by the embedded word-frequency lexicon.
pub struct SpellCorrector {
    word_freq: HashMap<String, u64>,
    total: u64,
}

impl SpellCorrector {
    /// Creates a corrector from the embedded lexicon.
    pub fn new() -> Self {
        let entries = parse_words(WORDS_DAT);
        let total: u64 = entries.values().map(|e| e.frequency).sum();
        let word_freq: HashMap<String, u64> =
            entries.into_iter().map(|(w, e)| (w, e.frequency)).collect();
        Self { word_freq, total }
    }

    /// Returns `true` if `word` is in the vocabulary.
    pub fn known(&self, word: &str) -> bool {
        self.word_freq.contains_key(word)
    }

    /// Unigram probability of `word` in the corpus.
    pub fn probability(&self, word: &str) -> f64 {
        let freq = self.word_freq.get(word).copied().unwrap_or(0);
        freq as f64 / self.total.max(1) as f64
    }

    /// Returns the most probable correction for `word`.
    ///
    /// Returns `word` unchanged if no better candidate is found.
    pub fn correction(&self, word: &str) -> String {
        self.candidates(word)
            .into_iter()
            .max_by(|a, b| {
                self.probability(a)
                    .partial_cmp(&self.probability(b))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or_else(|| word.to_string())
    }

    /// Returns all known candidate corrections for `word`.
    ///
    /// Returns `[word]` if it is already known.  Otherwise returns all known
    /// words within edit distance 1, or within edit distance 2 if there are
    /// none at distance 1.  Falls back to `[word]` if nothing is found.
    pub fn candidates(&self, word: &str) -> Vec<String> {
        if self.known(word) {
            return vec![word.to_string()];
        }

        let e1 = self.edits1(word);
        let known1: Vec<String> = e1.iter().filter(|w| self.known(w)).cloned().collect();
        if !known1.is_empty() {
            return known1;
        }

        let known2: HashSet<String> = e1
            .iter()
            .flat_map(|e| self.edits1(e))
            .filter(|w| self.known(w))
            .collect();
        if !known2.is_empty() {
            return known2.into_iter().collect();
        }

        vec![word.to_string()]
    }

    /// Generates all strings within edit distance 1 of `word`.
    ///
    /// Operations: deletion, transposition, replacement, insertion — all
    /// on Unicode characters (not bytes).
    pub fn edits1(&self, word: &str) -> HashSet<String> {
        let chars: Vec<char> = word.chars().collect();
        let n = chars.len();
        let mut out = HashSet::new();

        // Deletions
        for i in 0..n {
            out.insert(chars[..i].iter().chain(&chars[i + 1..]).collect());
        }

        // Transpositions
        for i in 0..n.saturating_sub(1) {
            let mut c = chars.clone();
            c.swap(i, i + 1);
            out.insert(c.iter().collect());
        }

        // Replacements
        for i in 0..n {
            for &ch in LETTERS {
                if ch != chars[i] {
                    out.insert(
                        chars[..i].iter().chain(std::iter::once(&ch)).chain(&chars[i + 1..]).collect(),
                    );
                }
            }
        }

        // Insertions
        for i in 0..=n {
            for &ch in LETTERS {
                out.insert(
                    chars[..i].iter().chain(std::iter::once(&ch)).chain(&chars[i..]).collect(),
                );
            }
        }

        out
    }
}

impl Default for SpellCorrector {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_word_unchanged() {
        let spell = SpellCorrector::new();
        // "کتاب" should be in the lexicon and come back as-is
        let result = spell.correction("کتاب");
        assert_eq!(result, "کتاب");
    }

    #[test]
    fn known_returns_true_for_common_word() {
        let spell = SpellCorrector::new();
        assert!(spell.known("کتاب"));
    }

    #[test]
    fn unknown_word_has_candidates_or_falls_through() {
        let spell = SpellCorrector::new();
        // xyz is not Persian — candidates should just return ["xyz"]
        let cands = spell.candidates("xyz");
        assert_eq!(cands, vec!["xyz"]);
    }

    #[test]
    fn edits1_contains_deletion() {
        let spell = SpellCorrector::new();
        // Deleting the first char of "کتاب" gives "تاب"
        let edits = spell.edits1("کتاب");
        assert!(edits.contains("تاب"));
    }

    #[test]
    fn probability_known_word_positive() {
        let spell = SpellCorrector::new();
        assert!(spell.probability("کتاب") > 0.0);
    }

    #[test]
    fn probability_unknown_word_zero() {
        let spell = SpellCorrector::new();
        assert_eq!(spell.probability("xyzxyz"), 0.0);
    }
}

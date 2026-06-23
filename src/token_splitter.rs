//! Splits a compound token into valid sub-word pairs using the lexicon.

use crate::lemmatizer::Lemmatizer;

const ZWNJ: char = '\u{200C}';

/// Splits compound Persian tokens into their constituent words.
///
/// Uses the word lexicon (via [`Lemmatizer`]) to validate candidate splits.
/// If a token can be split in more than one way every valid split is returned.
///
/// # Examples
///
/// ```
/// use rustam::TokenSplitter;
///
/// let splitter = TokenSplitter::new();
/// assert_eq!(splitter.split("صداوسیماجمهوری"), vec![vec!["صداوسیما", "جمهوری"]]);
/// ```
pub struct TokenSplitter {
    lemmatizer: Lemmatizer,
}

impl TokenSplitter {
    /// Creates a `TokenSplitter` backed by the default embedded lexicon.
    pub fn new() -> Self {
        Self {
            lemmatizer: Lemmatizer::new(),
        }
    }

    /// Returns all valid ways to split `token` into lexicon words.
    ///
    /// Each returned item is a `Vec<String>` of the split parts.
    /// The full unsplit token is included when it is itself in the lexicon.
    pub fn split(&self, token: &str) -> Vec<Vec<String>> {
        let mut candidates: Vec<Vec<String>> = Vec::new();

        // ZWNJ-boundary split: "داستان‌سرا" → ["داستان", "سرا"]
        if token.contains(ZWNJ) {
            let parts: Vec<String> = token
                .split(ZWNJ)
                .map(str::to_string)
                .collect();
            if self.all_in_lexicon(&parts) {
                candidates.push(parts);
            }
        }

        // Enumerate all possible binary splits that don't cut across a ZWNJ
        let chars: Vec<char> = token.chars().collect();
        let n = chars.len();

        for split_at in 1..n {
            // Don't split immediately after or before a ZWNJ
            if chars[split_at - 1] == ZWNJ || chars[split_at] == ZWNJ {
                continue;
            }
            let left: String = chars[..split_at].iter().collect();
            let right: String = chars[split_at..].iter().collect();
            let pair = vec![left, right];
            if self.all_in_lexicon(&pair) {
                candidates.push(pair);
            }
        }

        // Whole token as-is (if in lexicon)
        let whole = vec![token.to_string()];
        if self.all_in_lexicon(&whole) && !candidates.iter().any(|c| c == &whole) {
            candidates.push(whole);
        }

        candidates
    }

    fn all_in_lexicon(&self, parts: &[String]) -> bool {
        parts.iter().all(|p| {
            let lemma = self.lemmatizer.lemmatize(p, "");
            self.lemmatizer.words.contains_key(&lemma)
                || self.lemmatizer.words.contains_key(p.as_str())
        })
    }
}

impl Default for TokenSplitter {
    fn default() -> Self {
        Self::new()
    }
}

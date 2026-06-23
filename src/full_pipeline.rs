//! All-in-one Persian NLP pipeline.
//!
//! [`Rustam`] is a convenience wrapper that initialises the core
//! components once and exposes them through a single object.
//!
//! # Example
//!
//! ```
//! use rustam::Rustam;
//!
//! let nlp = Rustam::new();
//!
//! let text = "خانه‌ای که در آن زندگی می‌کنم بزرگ است.";
//! let normalized  = nlp.normalize(text);
//! let sentences   = nlp.sentences(text);
//! let words       = nlp.words(text);
//! let stem        = nlp.stem("کتاب‌ها");
//! let lemma       = nlp.lemmatize("می‌روم", "V");
//! ```

use crate::{Lemmatizer, Normalizer, SentenceTokenizer, Stemmer, WordTokenizer};

/// All-in-one Persian NLP pipeline.
///
/// Bundles the five always-available components — [`Normalizer`],
/// [`SentenceTokenizer`], [`WordTokenizer`], [`Stemmer`], [`Lemmatizer`] —
/// behind a single struct so callers don't need to manage each separately.
///
/// Feature-gated components (POS tagger, NER, dependency parser) are accessed
/// through their own structs (`POSTagger`, `NerTagger`, `DependencyParser`).
pub struct Rustam {
    normalizer:      Normalizer,
    sent_tokenizer:  SentenceTokenizer,
    word_tokenizer:  WordTokenizer,
    stemmer:         Stemmer,
    lemmatizer:      Lemmatizer,
}

impl Rustam {
    /// Creates a pipeline with default settings for all components.
    pub fn new() -> Self {
        Self {
            normalizer:     Normalizer::new(),
            sent_tokenizer: SentenceTokenizer::new(),
            word_tokenizer: WordTokenizer::new(),
            stemmer:        Stemmer::new(),
            lemmatizer:     Lemmatizer::new(),
        }
    }

    /// Normalises Persian text (character translation, spacing, diacritics, …).
    pub fn normalize(&self, text: &str) -> String {
        self.normalizer.normalize(text)
    }

    /// Splits `text` into sentences.
    pub fn sentences(&self, text: &str) -> Vec<String> {
        self.sent_tokenizer.tokenize(text)
    }

    /// Splits `text` into word tokens.
    pub fn words(&self, text: &str) -> Vec<String> {
        self.word_tokenizer.tokenize(text)
    }

    /// Normalises `text`, then splits into word tokens.
    pub fn word_tokenize_normalized(&self, text: &str) -> Vec<String> {
        self.words(&self.normalize(text))
    }

    /// Normalises `text`, splits into sentences, then word-tokenises each.
    pub fn tokenize_sents(&self, text: &str) -> Vec<Vec<String>> {
        self.sentences(&self.normalize(text))
            .into_iter()
            .map(|s| self.words(&s))
            .collect()
    }

    /// Returns the stem of `word` (rule-based suffix stripping).
    pub fn stem(&self, word: &str) -> String {
        self.stemmer.stem(word)
    }

    /// Returns the lemma of `word`.
    ///
    /// Pass the POS tag as `pos` (e.g. `"V"` for verb, `"N"` for noun) for
    /// better accuracy; pass `""` when unknown.
    pub fn lemmatize(&self, word: &str, pos: &str) -> String {
        self.lemmatizer.lemmatize(word, pos)
    }

    /// Returns a reference to the inner [`Normalizer`].
    pub fn normalizer(&self) -> &Normalizer { &self.normalizer }
    /// Returns a reference to the inner [`SentenceTokenizer`].
    pub fn sent_tokenizer(&self) -> &SentenceTokenizer { &self.sent_tokenizer }
    /// Returns a reference to the inner [`WordTokenizer`].
    pub fn word_tokenizer(&self) -> &WordTokenizer { &self.word_tokenizer }
    /// Returns a reference to the inner [`Stemmer`].
    pub fn stemmer(&self) -> &Stemmer { &self.stemmer }
    /// Returns a reference to the inner [`Lemmatizer`].
    pub fn lemmatizer(&self) -> &Lemmatizer { &self.lemmatizer }
}

impl Default for Rustam {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipeline_normalize_words() {
        let nlp = Rustam::new();
        let words = nlp.word_tokenize_normalized("کتاب‌ها را می‌خوانم");
        assert!(!words.is_empty());
    }

    #[test]
    fn pipeline_sentences() {
        let nlp = Rustam::new();
        let sents = nlp.sentences("جمله اول. جمله دوم.");
        assert!(sents.len() >= 1);
    }

    #[test]
    fn pipeline_stem_lemma() {
        let nlp = Rustam::new();
        assert_eq!(nlp.stem("کتاب‌ها"), "کتاب");
        // lemmatize with empty pos just checks it doesn't panic
        let _ = nlp.lemmatize("می‌روم", "V");
    }

    #[test]
    fn pipeline_tokenize_sents() {
        let nlp = Rustam::new();
        let result = nlp.tokenize_sents("اول. دوم.");
        assert!(!result.is_empty());
    }
}

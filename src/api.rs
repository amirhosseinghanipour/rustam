//! Traits defining the standard Persian NLP pipeline interfaces.
//!
//! Implementing these traits lets components be used generically — e.g. a
//! function that accepts `impl Tokenizer` works with both `WordTokenizer` and
//! `SentenceTokenizer`.

use crate::types::{Sentence, TaggedSentence, Token};

/// Any component that normalizes Persian text.
pub trait Normalizer {
    /// Returns the normalized form of `text`.
    fn normalize(&self, text: &str) -> String;
}

/// Any component that splits text into tokens.
pub trait Tokenizer {
    /// Splits `text` into a list of tokens.
    fn tokenize(&self, text: &str) -> Vec<Token>;
}

/// Any component that produces the base form (lemma) of a word.
pub trait Lemmatizer {
    /// Returns the lemma of `word`.
    ///
    /// `pos` is an optional POS tag hint; pass `""` when unknown.
    fn lemmatize(&self, word: &str, pos: &str) -> String;
}

/// Any component that assigns POS tags to a tokenized sentence.
pub trait Tagger {
    /// Tags a single tokenized sentence.
    fn tag(&self, tokens: &Sentence) -> TaggedSentence;

    /// Tags multiple sentences.
    fn tag_sents(&self, sentences: &[Sentence]) -> Vec<TaggedSentence> {
        sentences.iter().map(|s| self.tag(s)).collect()
    }
}

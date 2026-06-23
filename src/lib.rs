//! # rustam
//!
//! **Rustam** is a full Persian NLP pipeline for Rust:
//! normalization, tokenization, stemming, lemmatization, conjugation,
//! informal normalization, corpus readers, spell correction, and embedding backends.
//!
//! 1. **Normalization** — character translation, diacritics removal, spacing fixes
//! 2. **Informal normalization** — colloquial-to-formal Persian conversion
//! 3. **Sentence tokenization** — boundary detection using punctuation patterns
//! 4. **Word tokenization** — splits words, joins multi-part verbs
//! 5. **Stemming** — rule-based suffix stripping
//! 6. **Lemmatization** — dictionary lookup with full conjugation table
//! 7. **Conjugation** — generates every tense/mood/voice form for a verb
//! 8. **Token splitting** — compound token decomposition using the lexicon
//! 9. **Corpus readers** — Bijankhan, Arman and more
//! 10. **Embedding traits** — `WordEmbedding` / `SentenceEmbedding` interface
//!
//! ## Quick-start
//!
//! ```
//! use rustam::{Normalizer, SentenceTokenizer, WordTokenizer, Stemmer, Lemmatizer};
//!
//! let text = "اِعلاممممم کَرد : « زمین لرزه ای به بُزرگیِ 6 دهم ریشتر ...»";
//!
//! // Normalize
//! let norm = Normalizer::new();
//! let normalized = norm.normalize(text);
//!
//! // Sentence split
//! let sent_tok = SentenceTokenizer::new();
//! let sentences = sent_tok.tokenize(&normalized);
//!
//! // Word tokenize
//! let word_tok = WordTokenizer::new();
//! let tokens = word_tok.tokenize(&normalized);
//!
//! // Stem
//! let stemmer = Stemmer::new();
//! assert_eq!(stemmer.stem("کتاب‌ها"), "کتاب");
//!
//! // Lemmatize
//! let lem = Lemmatizer::new();
//! assert_eq!(lem.lemmatize("می‌روم", ""), "رفت#رو");
//! ```

#![warn(missing_docs)]

/// Persian verb conjugation — generates every tense/mood/voice form.
pub mod conjugation;
/// Compile-time constants: suffix lists, punctuation sets.
pub mod constants;
/// Embedded data files (lexicon, verbs, stopwords) compiled into the binary.
pub mod data;
/// Library-wide error type and `Result` alias.
pub mod error;
/// Informal (colloquial) Persian normalization and lemmatization.
pub mod informal;
/// Dictionary-based lemmatizer with conjugation fallback.
pub mod lemmatizer;
/// Multi-step text normalizer (diacritics, spacing, Unicode).
pub mod normalizer;
/// Sentence boundary tokenizer for Persian text.
pub mod sentence_tokenizer;
/// Persian spell corrector based on lexicon frequency.
pub mod spell;
/// Rule-based suffix-stripping stemmer.
pub mod stemmer;
/// Compound token splitter using the word lexicon.
pub mod token_splitter;
/// Arabic–Persian word translation table.
pub mod translate;
/// Core type aliases (`Token`, `TaggedSentence`, etc.).
pub mod types;
/// Word tokenizer with optional verb-part joining.
pub mod word_tokenizer;

// Re-export the most commonly used public types at the crate root.
pub use conjugation::Conjugation;
pub use data::{past_roots, present_roots, stopwords_list, verbs_list, words_list};
pub use error::{Error, Result};
pub use informal::{InformalLemmatizer, InformalNormalizer};
pub use lemmatizer::Lemmatizer;
pub use normalizer::{Normalizer, NormalizerConfig};
pub use sentence_tokenizer::{sent_tokenize, SentenceTokenizer};
pub use spell::SpellCorrector;
pub use stemmer::Stemmer;
pub use token_splitter::TokenSplitter;
pub use types::{
    ChunkedSentence, ChunkedToken, IobTag, Sentence, Tag, TaggedSentence, TaggedToken, Token,
    VerbRoots, WordEntry,
};
pub use word_tokenizer::{word_tokenize, WordTokenizer, WordTokenizerConfig};

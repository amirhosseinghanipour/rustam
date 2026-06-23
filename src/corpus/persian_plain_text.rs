#![allow(missing_docs)]
//! Reader for raw Persian plain-text files using rustam tokenizers.

use std::path::PathBuf;

use crate::sentence_tokenizer::SentenceTokenizer;
use crate::word_tokenizer::WordTokenizer;

/// Reads raw Persian plain-text files, tokenizing with rustam's tokenizers.
pub struct PersianPlainTextReader {
    path: PathBuf,
}

impl PersianPlainTextReader {
    /// Opens the plain text file at `path`.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    /// Returns the raw text content of the file.
    pub fn raw_text(&self) -> Result<String, std::io::Error> {
        std::fs::read_to_string(&self.path)
    }

    /// Returns an iterator over sentences in the file.
    pub fn sents(&self) -> Box<dyn Iterator<Item = String> + '_> {
        let text = match self.raw_text() {
            Ok(t) => t,
            Err(e) => {
                eprintln!("PersianPlainTextReader: cannot read {:?}: {}", self.path, e);
                return Box::new(std::iter::empty());
            }
        };
        let tokenizer = SentenceTokenizer::new();
        let sents: Vec<String> = tokenizer.tokenize(&text);
        Box::new(sents.into_iter())
    }

    /// Returns an iterator over all words in the file.
    pub fn words(&self) -> Box<dyn Iterator<Item = String> + '_> {
        let text = match self.raw_text() {
            Ok(t) => t,
            Err(e) => {
                eprintln!("PersianPlainTextReader: cannot read {:?}: {}", self.path, e);
                return Box::new(std::iter::empty());
            }
        };
        let sent_tokenizer = SentenceTokenizer::new();
        let word_tokenizer = WordTokenizer::new();
        let sents = sent_tokenizer.tokenize(&text);
        let words: Vec<String> = sents
            .into_iter()
            .flat_map(|s| word_tokenizer.tokenize(&s))
            .collect();
        Box::new(words.into_iter())
    }
}

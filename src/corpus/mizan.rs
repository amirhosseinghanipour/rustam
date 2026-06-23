#![allow(missing_docs)]
//! Reader for the Mizan parallel Persian-English corpus.

use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter,
    path::PathBuf,
};

/// Reads the Mizan parallel Persian-English corpus.
pub struct MizanReader {
    root: PathBuf,
}

impl MizanReader {
    /// Opens the Mizan corpus directory.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    fn read_lines(&self, filename: &str) -> Vec<String> {
        let path = self.root.join(filename);
        let file = match File::open(&path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("MizanReader: cannot open {:?}: {}", path, e);
                return Vec::new();
            }
        };
        BufReader::new(file)
            .lines()
            .filter_map(|l| l.ok())
            .collect()
    }

    /// Returns an iterator over all Persian sentences.
    pub fn persian_sentences(&self) -> Box<dyn Iterator<Item = String> + '_> {
        let lines = self.read_lines("mizan_fa.txt");
        Box::new(lines.into_iter())
    }

    /// Returns an iterator over all English sentences.
    pub fn english_sentences(&self) -> Box<dyn Iterator<Item = String> + '_> {
        let lines = self.read_lines("mizan_en.txt");
        Box::new(lines.into_iter())
    }

    /// Returns an iterator over (English, Persian) sentence pairs.
    pub fn pairs(&self) -> Box<dyn Iterator<Item = (String, String)> + '_> {
        let fa_lines = self.read_lines("mizan_fa.txt");
        let en_lines = self.read_lines("mizan_en.txt");

        if fa_lines.is_empty() && en_lines.is_empty() {
            return Box::new(iter::empty());
        }

        let pairs: Vec<(String, String)> = en_lines
            .into_iter()
            .zip(fa_lines.into_iter())
            .collect();

        Box::new(pairs.into_iter())
    }
}

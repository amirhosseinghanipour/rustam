//! Reader for the [Arman NER corpus](https://github.com/HaniehP/PersianNER).
//!
//! The corpus is stored in IOB format: each line is `token label` (space
//! separated), with blank lines separating sentences.

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use crate::types::TaggedSentence;

use super::CorpusReader;

/// Reads the Arman Persian NER corpus.
///
/// The corpus directory contains multiple files named `train*.txt` and
/// `test*.txt`.  Specify the subset with [`ArmanReader::new`].
///
/// # Examples
///
/// ```no_run
/// use rustam::corpus::{ArmanReader, CorpusReader};
///
/// let reader = ArmanReader::new("arman/", "train");
/// for sent in reader.sents() {
///     println!("{:?}", sent);
/// }
/// ```
pub struct ArmanReader {
    paths: Vec<PathBuf>,
}

impl ArmanReader {
    /// Opens the Arman corpus in `dir`, selecting files that start with `subset`
    /// (`"train"` or `"test"`).
    pub fn new(dir: impl AsRef<Path>, subset: &str) -> Self {
        let pattern = subset.to_string();
        let paths = std::fs::read_dir(dir)
            .into_iter()
            .flatten()
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| {
                p.extension().map_or(false, |ext| ext == "txt")
                    && p.file_name()
                        .and_then(|n| n.to_str())
                        .map_or(false, |n| n.starts_with(&pattern))
            })
            .collect();
        Self { paths }
    }
}

impl CorpusReader for ArmanReader {
    fn sents(&self) -> Box<dyn Iterator<Item = TaggedSentence> + '_> {
        let paths = self.paths.clone();
        Box::new(ArmanIter::new(paths))
    }
}

struct ArmanIter {
    paths: Vec<PathBuf>,
    path_idx: usize,
    lines: Vec<String>,
    line_idx: usize,
}

impl ArmanIter {
    fn new(paths: Vec<PathBuf>) -> Self {
        let mut iter = Self {
            paths,
            path_idx: 0,
            lines: Vec::new(),
            line_idx: 0,
        };
        iter.load_next_file();
        iter
    }

    fn load_next_file(&mut self) {
        while self.path_idx < self.paths.len() {
            match File::open(&self.paths[self.path_idx]) {
                Ok(f) => {
                    self.lines = BufReader::new(f)
                        .lines()
                        .filter_map(|l| l.ok())
                        .collect();
                    self.line_idx = 0;
                    self.path_idx += 1;
                    return;
                }
                Err(e) => {
                    eprintln!("ArmanReader: cannot open {:?}: {}", self.paths[self.path_idx], e);
                    self.path_idx += 1;
                }
            }
        }
        self.lines = Vec::new();
        self.line_idx = usize::MAX;
    }
}

impl Iterator for ArmanIter {
    type Item = TaggedSentence;

    fn next(&mut self) -> Option<Self::Item> {
        let mut sentence: TaggedSentence = Vec::new();
        loop {
            if self.line_idx >= self.lines.len() {
                if !sentence.is_empty() {
                    return Some(sentence);
                }
                if self.path_idx >= self.paths.len() {
                    return None;
                }
                self.load_next_file();
                if self.lines.is_empty() {
                    return None;
                }
                continue;
            }

            let line = self.lines[self.line_idx].trim().to_string();
            self.line_idx += 1;

            if line.is_empty() {
                if !sentence.is_empty() {
                    return Some(sentence);
                }
            } else {
                let mut parts = line.splitn(2, ' ');
                if let (Some(token), Some(label)) = (parts.next(), parts.next()) {
                    sentence.push((token.to_string(), label.to_string()));
                }
            }
        }
    }
}

#![allow(missing_docs)]
//! Reader for the Naab Persian text corpus.

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

/// Reads the Naab Persian text corpus.
pub struct NaabReader {
    root: PathBuf,
    subset: String,
}

impl NaabReader {
    /// Opens the Naab corpus directory with optional subset filter.
    pub fn new(root: impl Into<PathBuf>, subset: impl Into<String>) -> Self {
        Self {
            root: root.into(),
            subset: subset.into(),
        }
    }

    /// Returns an iterator over all sentences.
    pub fn sents(&self) -> Box<dyn Iterator<Item = String> + '_> {
        let files: Vec<_> = match std::fs::read_dir(&self.root) {
            Ok(entries) => entries
                .flatten()
                .map(|e| e.path())
                .filter(|p| {
                    let fname = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    self.subset == "all" || fname.starts_with(self.subset.as_str())
                })
                .collect(),
            Err(e) => {
                eprintln!("NaabReader: cannot read dir {:?}: {}", self.root, e);
                return Box::new(std::iter::empty());
            }
        };

        let mut sents: Vec<String> = Vec::new();

        for file_path in &files {
            let file = match File::open(file_path) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("NaabReader: cannot open {:?}: {}", file_path, e);
                    continue;
                }
            };
            for line in BufReader::new(file).lines().filter_map(|l| l.ok()) {
                let s = line.trim().to_string();
                if !s.is_empty() {
                    sents.push(s);
                }
            }
        }

        Box::new(sents.into_iter())
    }
}

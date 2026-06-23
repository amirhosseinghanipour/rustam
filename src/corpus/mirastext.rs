#![allow(missing_docs)]
//! Reader for the MirasText Persian corpus.

use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter,
    path::PathBuf,
};

/// A document from the MirasText corpus.
pub struct MirasDoc {
    pub text: String,
}

/// Reads the MirasText Persian corpus (plain text, `***`-delimited fields).
pub struct MirasTextReader {
    path: PathBuf,
}

impl MirasTextReader {
    /// Opens the MirasText corpus file.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    /// Returns an iterator over all documents.
    pub fn docs(&self) -> Box<dyn Iterator<Item = MirasDoc> + '_> {
        let file = match File::open(&self.path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("MirasTextReader: cannot open {:?}: {}", self.path, e);
                return Box::new(iter::empty());
            }
        };

        let docs: Vec<MirasDoc> = BufReader::new(file)
            .lines()
            .filter_map(|l| l.ok())
            .filter_map(|line| {
                let text = line.splitn(2, "***").next()?.to_string();
                if text.is_empty() { None } else { Some(MirasDoc { text }) }
            })
            .collect();

        Box::new(docs.into_iter())
    }

    /// Returns an iterator over all document texts.
    pub fn texts(&self) -> Box<dyn Iterator<Item = String> + '_> {
        Box::new(self.docs().map(|d| d.text))
    }
}

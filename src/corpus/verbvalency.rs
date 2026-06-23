#![allow(missing_docs)]
//! Reader for the Persian Verb Valency corpus (TSV format).

use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter,
    path::PathBuf,
};

/// A verb entry from the Verb Valency corpus.
pub struct Verb {
    pub past_light_verb: String,
    pub present_light_verb: String,
    pub prefix: String,
    pub nonverbal_element: String,
    pub preposition: String,
    pub valency: String,
}

/// Reads the Persian Verb Valency corpus (TSV format).
pub struct VerbValencyReader {
    path: PathBuf,
}

impl VerbValencyReader {
    /// Opens the Verb Valency corpus file.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    /// Returns an iterator over all verb entries.
    pub fn verbs(&self) -> Box<dyn Iterator<Item = Verb> + '_> {
        let file = match File::open(&self.path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("VerbValencyReader: cannot open {:?}: {}", self.path, e);
                return Box::new(iter::empty());
            }
        };

        let lines: Vec<String> = BufReader::new(file)
            .lines()
            .filter_map(|l| l.ok())
            .collect();

        let mut verbs: Vec<Verb> = Vec::new();

        for (i, line) in lines.iter().enumerate() {
            // Skip header line (contains "بن ماضی")
            if line.contains("بن ماضی") {
                continue;
            }
            // Pre-process: replace "-\t" with "\t"
            let processed = line.replace("-\t", "\t");
            let cols: Vec<&str> = processed.split('\t').collect();
            if cols.len() < 6 {
                if i > 0 {
                    // Only warn for non-empty lines
                    let trimmed = line.trim();
                    if !trimmed.is_empty() {
                        // Skip silently — some lines may be malformed
                    }
                }
                continue;
            }
            verbs.push(Verb {
                past_light_verb: cols[0].to_string(),
                present_light_verb: cols[1].to_string(),
                prefix: cols[2].to_string(),
                nonverbal_element: cols[3].to_string(),
                preposition: cols[4].to_string(),
                valency: cols[5].to_string(),
            });
        }

        Box::new(verbs.into_iter())
    }
}

#![allow(missing_docs)]
//! Reader for the FaSpell Persian spell-checking corpus.

use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter,
    path::PathBuf,
};

/// A spell-check entry from the FaSpell corpus.
pub struct SpellEntry {
    pub misspelled: String,
    pub corrected: String,
    pub error_category: Option<u32>,
}

/// Reads the FaSpell Persian spell-checking corpus.
pub struct FaSpellReader {
    root: PathBuf,
}

impl FaSpellReader {
    /// Opens the FaSpell corpus directory.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    fn read_entries(&self, filename: &str, has_category: bool) -> Box<dyn Iterator<Item = SpellEntry> + '_> {
        let path = self.root.join(filename);
        let file = match File::open(&path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("FaSpellReader: cannot open {:?}: {}", path, e);
                return Box::new(iter::empty());
            }
        };

        let lines: Vec<String> = BufReader::new(file)
            .lines()
            .filter_map(|l| l.ok())
            .collect();

        let entries: Vec<SpellEntry> = lines
            .into_iter()
            .skip(1) // skip header
            .filter_map(move |line| {
                let cols: Vec<&str> = line.splitn(3, '\t').collect();
                if cols.len() < 2 {
                    return None;
                }
                let misspelled = cols[0].to_string();
                let corrected = cols[1].to_string();
                let error_category = if has_category && cols.len() >= 3 {
                    cols[2].trim().parse::<u32>().ok()
                } else {
                    None
                };
                Some(SpellEntry { misspelled, corrected, error_category })
            })
            .collect();

        Box::new(entries.into_iter())
    }

    /// Returns an iterator over main corpus entries (misspelled, corrected, category).
    pub fn main_entries(&self) -> Box<dyn Iterator<Item = SpellEntry> + '_> {
        self.read_entries("faspell_main.txt", true)
    }

    /// Returns an iterator over OCR corpus entries (misspelled, corrected).
    pub fn ocr_entries(&self) -> Box<dyn Iterator<Item = SpellEntry> + '_> {
        self.read_entries("faspell_ocr.txt", false)
    }
}

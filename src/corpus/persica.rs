#![allow(missing_docs)]
//! Reader for the Persica Persian news corpus (CSV-like, 7 lines per record).

use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter,
    path::PathBuf,
};

/// A document from the Persica corpus.
pub struct PersicaDoc {
    pub id: i64,
    pub title: String,
    pub text: String,
    pub date: String,
    pub time: String,
    pub category: String,
    pub category2: String,
}

/// Reads the Persica Persian news corpus.
pub struct PersicaReader {
    path: PathBuf,
}

impl PersicaReader {
    /// Opens the Persica corpus file.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    /// Returns an iterator over all documents.
    pub fn docs(&self) -> Box<dyn Iterator<Item = PersicaDoc> + '_> {
        let file = match File::open(&self.path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("PersicaReader: cannot open {:?}: {}", self.path, e);
                return Box::new(iter::empty());
            }
        };

        let mut lines: Vec<String> = BufReader::new(file)
            .lines()
            .filter_map(|l| l.ok())
            .collect();

        // Strip BOM from first line if present
        if let Some(first) = lines.first_mut() {
            if first.starts_with('\u{FEFF}') {
                *first = first.trim_start_matches('\u{FEFF}').to_string();
            }
        }

        let mut docs: Vec<PersicaDoc> = Vec::new();
        let mut i = 0;
        while i + 6 < lines.len() {
            let chunk = &lines[i..i + 7];

            fn strip_trailing_comma(s: &str) -> String {
                s.trim_end_matches(',').to_string()
            }

            let id_str = strip_trailing_comma(chunk[0].trim());
            let id: i64 = id_str.parse().unwrap_or(0);
            let title = strip_trailing_comma(chunk[1].trim());
            let text = strip_trailing_comma(chunk[2].trim());
            let date = strip_trailing_comma(chunk[3].trim());
            let time = strip_trailing_comma(chunk[4].trim());
            let category = strip_trailing_comma(chunk[5].trim());
            let category2 = strip_trailing_comma(chunk[6].trim());

            docs.push(PersicaDoc {
                id,
                title,
                text,
                date,
                time,
                category,
                category2,
            });

            i += 7;
        }

        Box::new(docs.into_iter())
    }
}

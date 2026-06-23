#![allow(missing_docs)]
//! Reader for pre-extracted Wikipedia dumps (wiki_extractor format).

use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter,
    path::PathBuf,
};

/// A document from the Wikipedia dump.
pub struct WikiDoc {
    pub id: String,
    pub url: String,
    pub title: String,
    pub text: String,
}

fn extract_attr<'a>(line: &'a str, attr: &str) -> &'a str {
    let search = format!("{}=\"", attr);
    if let Some(start) = line.find(&search) {
        let rest = &line[start + search.len()..];
        if let Some(end) = rest.find('"') {
            return &rest[..end];
        }
    }
    ""
}

/// Reads pre-extracted Wikipedia text (wiki_extractor format).
pub struct WikipediaReader {
    path: PathBuf,
}

impl WikipediaReader {
    /// Opens the Wikipedia text dump file.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    /// Returns an iterator over all documents.
    pub fn docs(&self) -> Box<dyn Iterator<Item = WikiDoc> + '_> {
        let file = match File::open(&self.path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("WikipediaReader: cannot open {:?}: {}", self.path, e);
                return Box::new(iter::empty());
            }
        };

        let mut docs: Vec<WikiDoc> = Vec::new();
        let mut current_doc: Option<WikiDoc> = None;
        let mut text_lines: Vec<String> = Vec::new();

        for line in BufReader::new(file).lines().filter_map(|l| l.ok()) {
            if line.starts_with("<doc ") {
                let id = extract_attr(&line, "id").to_string();
                let url = extract_attr(&line, "url").to_string();
                let title = extract_attr(&line, "title").to_string();
                current_doc = Some(WikiDoc {
                    id,
                    url,
                    title,
                    text: String::new(),
                });
                text_lines.clear();
            } else if line == "</doc>" {
                if let Some(mut doc) = current_doc.take() {
                    doc.text = text_lines.join("\n");
                    docs.push(doc);
                }
                text_lines.clear();
            } else if current_doc.is_some() {
                text_lines.push(line);
            }
        }

        Box::new(docs.into_iter())
    }

    /// Returns an iterator over all document texts.
    pub fn texts(&self) -> Box<dyn Iterator<Item = String> + '_> {
        Box::new(self.docs().map(|d| d.text))
    }
}

#![allow(missing_docs)]
//! Reader for the PN Summary Persian news summarization corpus.

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

/// A document from the PN Summary corpus.
pub struct PnSummaryDoc {
    pub id: String,
    pub title: String,
    pub article: String,
    pub summary: String,
    pub category: String,
    pub categories: Vec<String>,
    pub network: String,
    pub link: String,
}

/// Reads the PN Summary Persian news summarization corpus.
pub struct PnSummaryReader {
    root: PathBuf,
    subset: String,
}

impl PnSummaryReader {
    /// Opens the PN Summary corpus directory with optional subset filter.
    pub fn new(root: impl Into<PathBuf>, subset: impl Into<String>) -> Self {
        Self {
            root: root.into(),
            subset: subset.into(),
        }
    }

    /// Returns an iterator over all documents.
    pub fn docs(&self) -> Box<dyn Iterator<Item = PnSummaryDoc> + '_> {
        let files: Vec<_> = match std::fs::read_dir(&self.root) {
            Ok(entries) => entries
                .flatten()
                .map(|e| e.path())
                .filter(|p| {
                    p.extension().map_or(false, |e| e == "csv") && {
                        let fname = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                        self.subset == "all" || fname.starts_with(self.subset.as_str())
                    }
                })
                .collect(),
            Err(e) => {
                eprintln!("PnSummaryReader: cannot read dir {:?}: {}", self.root, e);
                return Box::new(std::iter::empty());
            }
        };

        let mut docs: Vec<PnSummaryDoc> = Vec::new();

        for file_path in &files {
            let file = match File::open(file_path) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("PnSummaryReader: cannot open {:?}: {}", file_path, e);
                    continue;
                }
            };
            let mut first = true;
            for line in BufReader::new(file).lines().filter_map(|l| l.ok()) {
                // Skip header row
                if first {
                    first = false;
                    continue;
                }
                let cols: Vec<&str> = line.splitn(8, '\t').collect();
                if cols.len() < 8 {
                    continue;
                }
                let categories: Vec<String> = cols[4]
                    .split('+')
                    .map(|s| s.to_string())
                    .collect();
                docs.push(PnSummaryDoc {
                    id: cols[0].to_string(),
                    title: cols[1].to_string(),
                    article: cols[2].to_string(),
                    summary: cols[3].to_string(),
                    category: cols[4].to_string(),
                    categories,
                    network: cols[5].to_string(),
                    link: cols[6].to_string(),
                });
            }
        }

        Box::new(docs.into_iter())
    }
}

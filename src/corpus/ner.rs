#![allow(missing_docs)]
//! Reader for Persian NER corpora (BIO/IOB format).

use std::{
    path::{Path, PathBuf},
};

use crate::types::TaggedSentence;
use super::CorpusReader;

fn collect_txt_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(collect_txt_files(&path));
            } else if path.extension().map_or(false, |e| e == "txt") {
                files.push(path);
            }
        }
    }
    files
}

/// Reads Persian NER corpora in BIO/IOB format.
pub struct NerReader {
    root: PathBuf,
}

impl NerReader {
    /// Opens the NER corpus directory.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }
}

impl CorpusReader for NerReader {
    fn sents(&self) -> Box<dyn Iterator<Item = TaggedSentence> + '_> {
        let files = collect_txt_files(&self.root);
        let mut sentences: Vec<TaggedSentence> = Vec::new();

        for file_path in &files {
            let content = match std::fs::read_to_string(file_path) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("NerReader: cannot read {:?}: {}", file_path, e);
                    continue;
                }
            };

            let mut current: TaggedSentence = Vec::new();
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() {
                    if !current.is_empty() {
                        sentences.push(std::mem::take(&mut current));
                    }
                    continue;
                }
                let mut parts = line.splitn(2, '\t');
                if let (Some(token), Some(tag)) = (parts.next(), parts.next()) {
                    current.push((token.to_string(), tag.to_string()));
                }
            }
            if !current.is_empty() {
                sentences.push(current);
            }
        }

        Box::new(sentences.into_iter())
    }
}

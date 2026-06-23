#![allow(missing_docs)]
//! Reader for the Persian Treebank (XML format).

use std::{
    iter,
    path::{Path, PathBuf},
};

use quick_xml::Reader;
use quick_xml::events::Event;

use crate::types::TaggedSentence;
use super::CorpusReader;

fn coarse_pos_e(lc: &str, clitic: &str) -> String {
    let base = match lc.chars().next().unwrap_or(' ') {
        'N' => "N",
        'V' => "V",
        'A' => "AJ",
        'D' => "ADV",
        'Z' => "PRO",
        'T' => "DET",
        'E' => "P",
        'P' => "POSTP",
        'U' => "NUM",
        'J' => "CONJ",
        'O' => "PUNC",
        'R' => "RES",
        'L' => "CL",
        'I' => "INT",
        _ => "",
    };
    if clitic == "ezafe" {
        format!("{}e", base)
    } else {
        base.to_string()
    }
}

fn collect_xml_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(collect_xml_files(&path));
            } else if path.extension().map_or(false, |e| e == "xml") {
                files.push(path);
            }
        }
    }
    files
}

/// Reads the Persian Treebank XML corpus.
pub struct TreebankReader {
    root: PathBuf,
}

impl TreebankReader {
    /// Opens the Treebank corpus directory.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }
}

impl CorpusReader for TreebankReader {
    fn sents(&self) -> Box<dyn Iterator<Item = TaggedSentence> + '_> {
        let files = collect_xml_files(&self.root);
        let mut sentences: Vec<TaggedSentence> = Vec::new();

        for file_path in &files {
            let content = match std::fs::read_to_string(file_path) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("TreebankReader: cannot read {:?}: {}", file_path, e);
                    continue;
                }
            };

            let mut reader = Reader::from_str(&content);
            reader.config_mut().trim_text(true);
            let mut buf = Vec::new();

            let mut in_sentence = false;
            let mut current: TaggedSentence = Vec::new();
            let mut current_lc = String::new();
            let mut current_clitic = String::new();
            let mut in_word = false;
            let mut word_text = String::new();

            loop {
                match reader.read_event_into(&mut buf) {
                    Ok(Event::Start(e)) => {
                        match e.name().as_ref() {
                            b"S" => {
                                in_sentence = true;
                                current.clear();
                            }
                            b"w" if in_sentence => {
                                in_word = true;
                                word_text.clear();
                                current_lc.clear();
                                current_clitic.clear();
                                for attr in e.attributes().flatten() {
                                    let key = attr.key.as_ref();
                                    let val = String::from_utf8_lossy(&attr.value).into_owned();
                                    if key == b"lc" {
                                        current_lc = val;
                                    } else if key == b"clitic" {
                                        current_clitic = val;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    Ok(Event::End(e)) => {
                        match e.name().as_ref() {
                            b"w" if in_word => {
                                in_word = false;
                                let pos = coarse_pos_e(&current_lc, &current_clitic);
                                current.push((word_text.clone(), pos));
                            }
                            b"S" if in_sentence => {
                                in_sentence = false;
                                if !current.is_empty() {
                                    sentences.push(std::mem::take(&mut current));
                                }
                            }
                            _ => {}
                        }
                    }
                    Ok(Event::Text(e)) if in_word => {
                        let text = e.unescape().unwrap_or_default().into_owned();
                        word_text.push_str(&text);
                    }
                    Ok(Event::CData(e)) if in_word => {
                        let text = String::from_utf8_lossy(e.as_ref()).into_owned();
                        word_text.push_str(&text);
                    }
                    Ok(Event::Eof) => break,
                    Err(e) => {
                        eprintln!("TreebankReader: XML error in {:?}: {}", file_path, e);
                        break;
                    }
                    _ => {}
                }
                buf.clear();
            }
        }

        if sentences.is_empty() {
            return Box::new(iter::empty());
        }
        Box::new(sentences.into_iter())
    }
}

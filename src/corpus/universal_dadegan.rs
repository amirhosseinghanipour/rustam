#![allow(missing_docs)]
//! Reader for the Universal Dadegan Persian Dependency Treebank (CoNLL-U format).

use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter,
    path::PathBuf,
};

use crate::types::TaggedSentence;
use super::CorpusReader;

/// POS-tag coarsening mode for the Universal Dadegan reader.
pub enum UniversalDadeganMode {
    /// Coarse POS with ezafe marker (default).
    CoarsePosE,
    /// Universal Dependencies POS tags.
    CoarsePosU,
    /// Raw ctag+feats concatenation.
    Raw,
}

fn coarse_pos_e(ctag: &str, _tag: &str, feats: &str) -> String {
    let base = match ctag {
        "N" | "NOUN" => "N",
        "V" | "VERB" => "V",
        "ADJ" => "AJ",
        "ADV" => "ADV",
        "PR" | "PRON" => "PRO",
        "PREM" | "DET" => "DET",
        "PREP" | "ADP" => "P",
        "POSTP" => "POSTP",
        "PRENUM" | "NUM" => "NUM",
        "CONJ" | "CCONJ" => "CONJ",
        "PUNC" | "PUNCT" => "PUNC",
        "SUBR" | "SCONJ" => "CONJ",
        other => other,
    };
    if feats.contains("Ezafe") || feats.contains("Case=Ezafe") {
        format!("{}e", base)
    } else {
        base.to_string()
    }
}

fn coarse_pos_u(ctag: &str, word: &str) -> String {
    let base = match ctag {
        "N" | "NOUN" => "NOUN",
        "V" | "VERB" => "VERB",
        "ADJ" => "ADJ",
        "ADV" => "ADV",
        "PR" | "PRON" => "PRON",
        "PREM" | "DET" => "DET",
        "PREP" | "ADP" => "ADP",
        "POSTP" => "ADP",
        "PRENUM" | "NUM" => "NUM",
        "CONJ" | "CCONJ" => "CCONJ",
        "PUNC" | "PUNCT" => "PUNCT",
        "SUBR" | "SCONJ" => "SCONJ",
        "IDEN" | "PROPN" => "PROPN",
        "POSTNUM" => "NUM",
        "PSUS" | "INTJ" => "INTJ",
        "PART" => "PART",
        "ADR" => "INTJ",
        other => other,
    };
    if base == "PART" {
        if word == "را" || word == "خوب" || word == "آخر" {
            return "ADP".to_string();
        }
    }
    base.to_string()
}

/// Reads the Universal Dadegan Persian Dependency Treebank (CoNLL-U format).
pub struct UniversalDadeganReader {
    path: PathBuf,
    mode: UniversalDadeganMode,
}

impl UniversalDadeganReader {
    /// Opens the Universal Dadegan corpus in CoarsePosE mode (default).
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into(), mode: UniversalDadeganMode::CoarsePosE }
    }

    /// Opens the Universal Dadegan corpus in Universal POS mode.
    pub fn universal(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into(), mode: UniversalDadeganMode::CoarsePosU }
    }

    /// Opens the Universal Dadegan corpus in Raw mode (ctag,feats).
    pub fn raw_pos(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into(), mode: UniversalDadeganMode::Raw }
    }

    fn parse_pos(&self, ctag: &str, tag: &str, feats: &str, word: &str) -> String {
        match self.mode {
            UniversalDadeganMode::CoarsePosE => coarse_pos_e(ctag, tag, feats),
            UniversalDadeganMode::CoarsePosU => coarse_pos_u(ctag, word),
            UniversalDadeganMode::Raw => format!("{},{}", ctag, feats),
        }
    }
}

impl CorpusReader for UniversalDadeganReader {
    fn sents(&self) -> Box<dyn Iterator<Item = TaggedSentence> + '_> {
        let file = match File::open(&self.path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("UniversalDadeganReader: cannot open {:?}: {}", self.path, e);
                return Box::new(iter::empty());
            }
        };

        let lines: Vec<String> = BufReader::new(file)
            .lines()
            .filter_map(|l| l.ok())
            .collect();

        let mut sentences: Vec<TaggedSentence> = Vec::new();
        let mut current: TaggedSentence = Vec::new();

        for line in &lines {
            let line = line.trim();
            if line.is_empty() {
                if !current.is_empty() {
                    sentences.push(std::mem::take(&mut current));
                }
                continue;
            }
            // Skip comment lines
            if line.starts_with('#') {
                continue;
            }
            let cols: Vec<&str> = line.split('\t').collect();
            if cols.len() < 6 {
                continue;
            }
            let id = cols[0];
            // Skip multi-word tokens (id contains '-')
            if id.contains('-') {
                continue;
            }
            let word = cols[1].to_string();
            // col 3 = upos, col 4 = xpos, col 5 = feats
            let ctag = cols[3];
            let tag = cols[4];
            let feats = cols[5];

            let pos = self.parse_pos(ctag, tag, feats, &word);
            current.push((word, pos));
        }
        if !current.is_empty() {
            sentences.push(current);
        }

        Box::new(sentences.into_iter())
    }
}

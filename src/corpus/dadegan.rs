#![allow(missing_docs)]
//! Reader for the Dadegan Persian Dependency Treebank (CoNLL format).

use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter,
    path::PathBuf,
};

use crate::types::TaggedSentence;
use super::CorpusReader;

/// POS-tag coarsening mode for the Dadegan reader.
pub enum DadeganMode {
    /// Coarse POS with ezafe marker (default).
    CoarsePosE,
    /// Universal Dependencies POS tags.
    CoarsePosU,
    /// Raw ctag+feats concatenation.
    Raw,
}

fn coarse_pos_e(ctag: &str, _tag: &str, feats: &str) -> String {
    let base = match ctag {
        "N" => "N",
        "V" => "V",
        "ADJ" => "AJ",
        "ADV" => "ADV",
        "PR" => "PRO",
        "PREM" => "DET",
        "PREP" => "P",
        "POSTP" => "POSTP",
        "PRENUM" => "NUM",
        "CONJ" => "CONJ",
        "PUNC" => "PUNC",
        "SUBR" => "CONJ",
        other => other,
    };
    if feats.contains("ezafe") {
        format!("{}e", base)
    } else {
        base.to_string()
    }
}

fn coarse_pos_u(ctag: &str, word: &str) -> String {
    let base = match ctag {
        "N" => "NOUN",
        "V" => "VERB",
        "ADJ" => "ADJ",
        "ADV" => "ADV",
        "PR" => "PRON",
        "PREM" => "DET",
        "PREP" => "ADP",
        "POSTP" => "ADP",
        "PRENUM" => "NUM",
        "CONJ" => "CCONJ",
        "PUNC" => "PUNCT",
        "SUBR" => "SCONJ",
        "IDEN" => "PROPN",
        "POSTNUM" => "NUM",
        "PSUS" => "INTJ",
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

fn preprocess_line(line: &str) -> String {
    let zwnj = '\u{200C}';
    let s = line.replace('\r', "");
    let s = s.replace('\u{2029}', "\u{200C}");
    // Replace tab+ZWNJ and ZWNJ+tab with tab
    let s = s.replace(&format!("\t{}", zwnj), "\t");
    let s = s.replace(&format!("{}\t", zwnj), "\t");
    // Replace space+tab and tab+space with tab
    let s = s.replace(" \t", "\t");
    let s = s.replace("\t ", "\t");
    // Replace double ZWNJ with single ZWNJ
    let double_zwnj = format!("{}{}", zwnj, zwnj);
    s.replace(&double_zwnj, &zwnj.to_string())
}

/// Reads the Dadegan Persian Dependency Treebank (CoNLL format).
pub struct DadeganReader {
    path: PathBuf,
    mode: DadeganMode,
}

impl DadeganReader {
    /// Opens the Dadegan corpus in CoarsePosE mode (default).
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into(), mode: DadeganMode::CoarsePosE }
    }

    /// Opens the Dadegan corpus in Universal POS mode.
    pub fn universal(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into(), mode: DadeganMode::CoarsePosU }
    }

    /// Opens the Dadegan corpus in Raw mode (ctag,feats).
    pub fn raw_pos(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into(), mode: DadeganMode::Raw }
    }

    fn parse_pos(&self, ctag: &str, tag: &str, feats: &str, word: &str) -> String {
        match self.mode {
            DadeganMode::CoarsePosE => coarse_pos_e(ctag, tag, feats),
            DadeganMode::CoarsePosU => coarse_pos_u(ctag, word),
            DadeganMode::Raw => format!("{},{}", ctag, feats),
        }
    }
}

impl CorpusReader for DadeganReader {
    fn sents(&self) -> Box<dyn Iterator<Item = TaggedSentence> + '_> {
        let file = match File::open(&self.path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("DadeganReader: cannot open {:?}: {}", self.path, e);
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
            let line = preprocess_line(line);
            if line.trim().is_empty() {
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
            let mut word = cols[1].to_string();
            // Replace spaces in word with underscore
            word = word.replace(' ', "_");
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

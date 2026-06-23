#![allow(missing_docs)]
//! Reader for the Peykare Persian corpus (Windows-1256 encoded).

use std::path::{Path, PathBuf};

use crate::types::TaggedSentence;
use super::CorpusReader;

fn win1256_to_utf8(bytes: &[u8]) -> String {
    #[allow(clippy::unicode_not_nfc)]
    const TABLE: [char; 256] = [
        // 0x00-0x7F: ASCII
        '\x00', '\x01', '\x02', '\x03', '\x04', '\x05', '\x06', '\x07',
        '\x08', '\x09', '\x0A', '\x0B', '\x0C', '\x0D', '\x0E', '\x0F',
        '\x10', '\x11', '\x12', '\x13', '\x14', '\x15', '\x16', '\x17',
        '\x18', '\x19', '\x1A', '\x1B', '\x1C', '\x1D', '\x1E', '\x1F',
        ' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/',
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?',
        '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
        'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', '\\', ']', '^', '_',
        '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
        'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~', '\x7F',
        // 0x80-0xFF: Windows-1256 specific
        '\u{20AC}', '\u{067E}', '\u{201A}', '\u{0192}', '\u{201E}', '\u{2026}', '\u{2020}', '\u{2021}',
        '\u{02C6}', '\u{2030}', '\u{0679}', '\u{2039}', '\u{0152}', '\u{0686}', '\u{0698}', '\u{0688}',
        '\u{06AF}', '\u{2018}', '\u{2019}', '\u{201C}', '\u{201D}', '\u{2022}', '\u{2013}', '\u{2014}',
        '\u{06A9}', '\u{2122}', '\u{0691}', '\u{203A}', '\u{0153}', '\u{200C}', '\u{200D}', '\u{06BA}',
        '\u{00A0}', '\u{060C}', '\u{00A2}', '\u{00A3}', '\u{00A4}', '\u{00A5}', '\u{00A6}', '\u{00A7}',
        '\u{00A8}', '\u{00A9}', '\u{06BE}', '\u{00AB}', '\u{00AC}', '\u{00AD}', '\u{00AE}', '\u{00AF}',
        '\u{00B0}', '\u{00B1}', '\u{00B2}', '\u{00B3}', '\u{00B4}', '\u{00B5}', '\u{00B6}', '\u{00B7}',
        '\u{00B8}', '\u{00B9}', '\u{061B}', '\u{00BB}', '\u{00BC}', '\u{00BD}', '\u{00BE}', '\u{061F}',
        '\u{00C0}', '\u{0621}', '\u{0622}', '\u{0623}', '\u{0624}', '\u{0625}', '\u{0626}', '\u{0627}',
        '\u{0628}', '\u{0629}', '\u{062A}', '\u{062B}', '\u{062C}', '\u{062D}', '\u{062E}', '\u{062F}',
        '\u{0630}', '\u{0631}', '\u{0632}', '\u{0633}', '\u{0634}', '\u{0635}', '\u{0636}', '\u{00D7}',
        '\u{0637}', '\u{0638}', '\u{0639}', '\u{063A}', '\u{00DC}', '\u{00DD}', '\u{00DE}', '\u{0640}',
        '\u{0641}', '\u{0642}', '\u{0643}', '\u{0644}', '\u{0645}', '\u{0646}', '\u{0647}', '\u{0648}',
        '\u{0649}', '\u{064A}', '\u{064B}', '\u{064C}', '\u{064D}', '\u{064E}', '\u{064F}', '\u{0650}',
        '\u{0651}', '\u{0652}', '\u{0621}', '\u{0622}', '\u{0623}', '\u{0624}', '\u{00F6}', '\u{00F7}',
        '\u{0625}', '\u{0626}', '\u{0627}', '\u{0628}', '\u{0629}', '\u{064A}', '\u{0653}', '\u{0654}',
    ];
    bytes.iter().map(|&b| TABLE[b as usize]).collect()
}

fn coarse_pos_e(tags: &[&str]) -> String {
    const VALID: &[&str] = &[
        "N", "V", "AJ", "ADV", "PRO", "DET", "P", "POSTP", "NUM", "CONJ", "PUNC", "CL", "INT", "RES",
    ];
    let first = tags.iter().find(|&&t| VALID.contains(&t)).copied().unwrap_or("N");
    if tags.contains(&"EZ") {
        format!("{},EZ", first)
    } else {
        first.to_string()
    }
}

fn coarse_pos_u(tags: &[&str], word: &str) -> String {
    const VALID: &[&str] = &[
        "N", "V", "AJ", "ADV", "PRO", "DET", "P", "POSTP", "NUM", "CONJ", "PUNC", "CL", "INT", "RES",
    ];
    let first = tags.iter().find(|&&t| VALID.contains(&t)).copied().unwrap_or("N");
    let base = match first {
        "N" => "NOUN",
        "V" => "VERB",
        "AJ" => "ADJ",
        "ADV" => "ADV",
        "PRO" => "PRON",
        "DET" => "DET",
        "P" => "ADP",
        "POSTP" => "ADP",
        "NUM" => "NUM",
        "CONJ" => "CCONJ",
        "PUNC" => "PUNCT",
        "CL" => "PART",
        "INT" => "INTJ",
        "RES" => "X",
        other => other,
    };
    if base == "PART" {
        if word == "را" || word == "خوب" || word == "آخر" {
            return "ADP".to_string();
        }
    }
    base.to_string()
}

fn collect_files_recursive(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(collect_files_recursive(&path));
            } else {
                files.push(path);
            }
        }
    }
    files
}

/// Reads the Peykare Persian corpus (Windows-1256 encoded directory tree).
pub struct PeykareReader {
    root: PathBuf,
    universal: bool,
    join_verb_parts: bool,
}

impl PeykareReader {
    /// Opens the Peykare corpus directory.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            universal: false,
            join_verb_parts: true,
        }
    }

    /// Uses Universal Dependencies POS tags.
    pub fn with_universal(mut self) -> Self {
        self.universal = true;
        self
    }

    /// Disables multi-part verb joining.
    pub fn without_verb_joining(mut self) -> Self {
        self.join_verb_parts = false;
        self
    }
}

impl CorpusReader for PeykareReader {
    fn sents(&self) -> Box<dyn Iterator<Item = TaggedSentence> + '_> {
        let files = collect_files_recursive(&self.root);
        let mut sentences: Vec<TaggedSentence> = Vec::new();

        for file_path in &files {
            let bytes = match std::fs::read(file_path) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("PeykareReader: cannot read {:?}: {}", file_path, e);
                    continue;
                }
            };
            let content = win1256_to_utf8(&bytes);
            let mut current: TaggedSentence = Vec::new();

            for line in content.lines() {
                if line.trim().is_empty() {
                    continue;
                }
                let cols: Vec<&str> = line.split(' ').filter(|s| !s.is_empty()).collect();
                if cols.len() < 4 {
                    continue;
                }
                let coarse_pos = cols[2];
                let fine_tags_str = cols[3];
                let fine_tags: Vec<&str> = fine_tags_str.split(',').collect();
                let word_parts = &cols[4..];
                let zwnj = '\u{200C}';
                let word: String = word_parts.join(&zwnj.to_string());

                // Skip # words
                if word == "#" {
                    continue;
                }

                let pos = if self.universal {
                    coarse_pos_u(&fine_tags, &word)
                } else {
                    coarse_pos_e(&fine_tags)
                };

                current.push((word.clone(), pos));

                // Sentence boundary
                let boundary_words = ["#", ".", "؟", "!"];
                if coarse_pos == "PUNC" && boundary_words.contains(&word.as_str()) {
                    if current.len() > 1 {
                        sentences.push(std::mem::take(&mut current));
                    } else {
                        current.clear();
                    }
                }
            }
            if current.len() > 1 {
                sentences.push(current);
            }
        }

        Box::new(sentences.into_iter())
    }
}

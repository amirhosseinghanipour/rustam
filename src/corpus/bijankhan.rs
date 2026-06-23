//! Reader for the [Bijankhan corpus](https://www.peykaregan.ir/).
//!
//! The corpus is a tab-separated file where each line contains
//! `word  TAG` (two or more spaces as separator), with sentence boundaries
//! marked by delimiter tokens (`#`, `*`, `.`, `؟`, `!` tagged as `DELM`).

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use crate::{normalizer::Normalizer, types::TaggedSentence};

use super::CorpusReader;

/// Default coarse POS map that converts Bijankhan fine-grained tags to a
/// smaller universal-ish tagset.
pub fn default_pos_map() -> HashMap<&'static str, &'static str> {
    let pairs: &[(&str, &str)] = &[
        ("ADJ", "ADJ"), ("ADJ_CMPR", "ADJ"), ("ADJ_INO", "V"),
        ("ADJ_ORD", "ADJ"), ("ADJ_SIM", "ADJ"), ("ADJ_SUP", "ADJ"),
        ("ADV", "ADV"), ("ADV_EXM", "ADV"), ("ADV_I", "ADV"),
        ("ADV_NEGG", "ADV"), ("ADV_NI", "ADV"), ("ADV_TIME", "ADV"),
        ("AR", "AR"), ("CON", "CONJ"), ("DEFAULT", "DEFAULT"),
        ("DELM", "PUNC"), ("DET", "DET"), ("IF", "CONJ"),
        ("INT", "INT"), ("MORP", "MORP"), ("MQUA", "ADV"),
        ("MS", "MS"), ("N_PL", "N"), ("N_SING", "N"),
        ("NN", "NN"), ("NP", "NP"), ("OH", "OH"), ("OHH", "N"),
        ("P", "PREP"), ("PP", "PP"), ("PRO", "PR"), ("PS", "PS"),
        ("QUA", "DET"), ("SPEC", "SPEC"),
        ("V_AUX", "V"), ("V_IMP", "V"), ("V_PA", "V"),
        ("V_PRE", "V"), ("V_PRS", "V"), ("V_SUB", "V"),
    ];
    pairs.iter().copied().collect()
}

/// Reads the Bijankhan POS-tagged corpus.
///
/// # Examples
///
/// ```no_run
/// use rustam::corpus::{BijankhanReader, CorpusReader};
///
/// let reader = BijankhanReader::new("bijankhan.txt");
/// for sent in reader.sents() {
///     println!("{:?}", sent);
/// }
/// ```
pub struct BijankhanReader {
    path: PathBuf,
    pos_map: HashMap<&'static str, &'static str>,
    join_verb_parts: bool,
}

impl BijankhanReader {
    /// Opens the Bijankhan corpus at `path`.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            pos_map: default_pos_map(),
            join_verb_parts: true,
        }
    }

    /// Disables multi-part verb joining.
    pub fn without_verb_joining(mut self) -> Self {
        self.join_verb_parts = false;
        self
    }

    /// Uses a custom POS map instead of the default.
    pub fn with_pos_map(mut self, map: HashMap<&'static str, &'static str>) -> Self {
        self.pos_map = map;
        self
    }
}

impl CorpusReader for BijankhanReader {
    fn sents(&self) -> Box<dyn Iterator<Item = TaggedSentence> + '_> {
        let file = match File::open(&self.path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("BijankhanReader: cannot open {:?}: {}", self.path, e);
                return Box::new(std::iter::empty());
            }
        };
        let norm = Normalizer::with_config(crate::normalizer::NormalizerConfig {
            correct_spacing: false,
            ..Default::default()
        });
        let lines: Vec<String> = BufReader::new(file)
            .lines()
            .filter_map(|l| l.ok())
            .collect();

        let pos_map = self.pos_map.clone();
        let join = self.join_verb_parts;

        Box::new(BijankhanIter {
            lines,
            cursor: 0,
            norm,
            pos_map,
            join_verb_parts: join,
        })
    }
}

struct BijankhanIter {
    lines: Vec<String>,
    cursor: usize,
    norm: Normalizer,
    pos_map: HashMap<&'static str, &'static str>,
    join_verb_parts: bool,
}

impl Iterator for BijankhanIter {
    type Item = TaggedSentence;

    fn next(&mut self) -> Option<Self::Item> {
        let mut sentence: TaggedSentence = Vec::new();
        while self.cursor < self.lines.len() {
            let line = &self.lines[self.cursor];
            self.cursor += 1;
            let parts: Vec<&str> = line.trim().splitn(2, "  ").collect();
            if parts.len() < 2 {
                continue;
            }
            let word_raw = parts[0].trim();
            let tag_raw = parts[1].trim();

            if word_raw == "#" || word_raw == "*" {
                if tag_raw == "DELM" && !sentence.is_empty() {
                    let sent = std::mem::take(&mut sentence);
                    return Some(finalize(sent, &self.pos_map, self.join_verb_parts));
                }
                continue;
            }

            let word = self.norm.normalize(word_raw);
            let word = if word.is_empty() { "_".to_string() } else { word };
            sentence.push((word.clone(), tag_raw.to_string()));

            // Sentence boundary
            if tag_raw == "DELM" && [".", "؟", "!"].contains(&word.as_str()) && !sentence.is_empty() {
                let sent = std::mem::take(&mut sentence);
                return Some(finalize(sent, &self.pos_map, self.join_verb_parts));
            }
        }
        if !sentence.is_empty() {
            let sent = std::mem::take(&mut sentence);
            return Some(finalize(sent, &self.pos_map, self.join_verb_parts));
        }
        None
    }
}

fn finalize(
    sent: TaggedSentence,
    pos_map: &HashMap<&'static str, &'static str>,
    join: bool,
) -> TaggedSentence {
    let mapped: TaggedSentence = sent
        .into_iter()
        .map(|(w, t)| {
            let mapped_tag = pos_map.get(t.as_str()).copied().unwrap_or(t.as_str());
            (w, mapped_tag.to_string())
        })
        .collect();

    if join {
        join_verb_parts_tagged(mapped)
    } else {
        mapped
    }
}

/// Joins multi-part verbs in a tagged sentence (e.g. `دیده شد` → `دیده_شد`).
fn join_verb_parts_tagged(sent: TaggedSentence) -> TaggedSentence {
    // Simple heuristic: join consecutive V tokens with underscore
    let mut result: TaggedSentence = Vec::new();
    for (word, tag) in sent {
        let should_join = result.last().map_or(false, |(_, prev_tag): &(String, String)| {
            prev_tag == "V" && tag == "V"
        });
        if should_join {
            if let Some(last) = result.last_mut() {
                last.0 = format!("{}_{}", last.0, word);
            }
        } else {
            result.push((word, tag));
        }
    }
    result
}

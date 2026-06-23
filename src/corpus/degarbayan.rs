#![allow(missing_docs)]
//! Reader for the Degarbayan Persian paraphrase corpus (XML format).

use std::{
    iter,
    path::PathBuf,
};

use quick_xml::Reader;
use quick_xml::events::Event;

/// A paraphrase pair from the Degarbayan corpus.
pub struct ParaphrasePair {
    pub id: String,
    pub source1: String,
    pub source2: String,
    pub sentence1: String,
    pub sentence2: String,
    pub method_type: String,
    pub judge: String,
}

/// Reads the Degarbayan Persian paraphrase corpus (XML format).
pub struct DegarbayanReader {
    root: PathBuf,
    two_class: bool,
}

impl DegarbayanReader {
    /// Opens the Degarbayan corpus directory (three-class mode).
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into(), two_class: false }
    }

    /// Opens the Degarbayan corpus directory (two-class mode: "0" → "Paraphrase").
    pub fn new_two_class(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into(), two_class: true }
    }

    fn judge_label(&self, raw: &str) -> String {
        match raw {
            "1" => "Paraphrase".to_string(),
            "0" => {
                if self.two_class {
                    "Paraphrase".to_string()
                } else {
                    "SemiParaphrase".to_string()
                }
            }
            "-1" => "NotParaphrase".to_string(),
            other => other.to_string(),
        }
    }

    /// Returns an iterator over all paraphrase pairs.
    pub fn docs(&self) -> Box<dyn Iterator<Item = ParaphrasePair> + '_> {
        let xml_path = self.root.join("corpus_pair.xml");
        let content = match std::fs::read_to_string(&xml_path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("DegarbayanReader: cannot read {:?}: {}", xml_path, e);
                return Box::new(iter::empty());
            }
        };

        let mut pairs: Vec<ParaphrasePair> = Vec::new();
        let mut reader = Reader::from_str(&content);
        reader.config_mut().trim_text(true);
        let mut buf = Vec::new();

        // State
        let mut in_pair = false;
        let mut current_field = String::new();
        let mut current_text = String::new();
        // Fields for current pair
        let mut pair_id = String::new();
        let mut source1 = String::new();
        let mut source2 = String::new();
        let mut _news_id1 = String::new();
        let mut _news_id2 = String::new();
        let mut sentence1 = String::new();
        let mut sentence2 = String::new();
        let mut method_type = String::new();
        let mut judge_raw = String::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => {
                    let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                    match name.as_str() {
                        "Pair" => {
                            in_pair = true;
                            pair_id.clear();
                            source1.clear();
                            source2.clear();
                            _news_id1.clear();
                            _news_id2.clear();
                            sentence1.clear();
                            sentence2.clear();
                            method_type.clear();
                            judge_raw.clear();
                        }
                        _ if in_pair => {
                            current_field = name;
                            current_text.clear();
                        }
                        _ => {}
                    }
                }
                Ok(Event::End(e)) => {
                    let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                    match name.as_str() {
                        "Pair" => {
                            let judge = self.judge_label(&judge_raw);
                            pairs.push(ParaphrasePair {
                                id: pair_id.clone(),
                                source1: source1.clone(),
                                source2: source2.clone(),
                                sentence1: sentence1.clone(),
                                sentence2: sentence2.clone(),
                                method_type: method_type.clone(),
                                judge,
                            });
                            in_pair = false;
                        }
                        _ if in_pair => {
                            match current_field.as_str() {
                                "PairId" => pair_id = current_text.trim().to_string(),
                                "NewsSource1" => source1 = current_text.trim().to_string(),
                                "NewsSource2" => source2 = current_text.trim().to_string(),
                                "NewsId1" => _news_id1 = current_text.trim().to_string(),
                                "NewsId2" => _news_id2 = current_text.trim().to_string(),
                                "Sentence1" => sentence1 = current_text.trim().to_string(),
                                "Sentence2" => sentence2 = current_text.trim().to_string(),
                                "MethodType" => method_type = current_text.trim().to_string(),
                                "judge" => judge_raw = current_text.trim().to_string(),
                                _ => {}
                            }
                            current_field.clear();
                            current_text.clear();
                        }
                        _ => {}
                    }
                }
                Ok(Event::Text(e)) if in_pair => {
                    let text = e.unescape().unwrap_or_default().into_owned();
                    current_text.push_str(&text);
                }
                Ok(Event::CData(e)) if in_pair => {
                    let text = String::from_utf8_lossy(e.as_ref()).into_owned();
                    current_text.push_str(&text);
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    eprintln!("DegarbayanReader: XML error: {}", e);
                    break;
                }
                _ => {}
            }
            buf.clear();
        }

        Box::new(pairs.into_iter())
    }

    /// Returns an iterator over (sentence1, sentence2, judge) triples.
    pub fn pairs(&self) -> Box<dyn Iterator<Item = (String, String, String)> + '_> {
        let docs: Vec<ParaphrasePair> = self.docs().collect();
        let result: Vec<(String, String, String)> = docs
            .into_iter()
            .map(|p| (p.sentence1, p.sentence2, p.judge))
            .collect();
        Box::new(result.into_iter())
    }
}

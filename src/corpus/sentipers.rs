#![allow(missing_docs)]
//! Reader for the SentiPers Persian sentiment corpus (XML format).

use std::{
    path::{Path, PathBuf},
};

use quick_xml::Reader;
use quick_xml::events::Event;

/// A sentence from the SentiPers corpus.
pub struct SentiSentence {
    pub id: String,
    pub value: String,
    pub text: String,
}

/// A comment (Opinion or Criticism) from the SentiPers corpus.
pub struct SentiComment {
    pub id: String,
    pub holder: String,
    pub value: String,
    pub sentences: Vec<SentiSentence>,
}

/// A product document from the SentiPers corpus.
pub struct SentiDoc {
    pub title: String,
    pub product_type: String,
    pub comments: Vec<SentiComment>,
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

fn parse_senti_docs(content: &str) -> Vec<SentiDoc> {
    let mut docs: Vec<SentiDoc> = Vec::new();
    let mut reader = Reader::from_str(content);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();

    let mut in_product = false;
    let mut current_doc: Option<SentiDoc> = None;
    let mut in_comment = false;
    let mut current_comment: Option<SentiComment> = None;
    let mut in_sentence = false;
    let mut current_sentence: Option<SentiSentence> = None;
    let mut current_text = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                match name.as_str() {
                    "Product" => {
                        in_product = true;
                        let mut doc = SentiDoc {
                            title: String::new(),
                            product_type: String::new(),
                            comments: Vec::new(),
                        };
                        for attr in e.attributes().flatten() {
                            let key = attr.key.as_ref();
                            let val = String::from_utf8_lossy(&attr.value).into_owned();
                            if key == b"Title" {
                                doc.title = val;
                            } else if key == b"Type" {
                                doc.product_type = val;
                            }
                        }
                        current_doc = Some(doc);
                    }
                    "Opinion" | "Criticism" if in_product => {
                        in_comment = true;
                        let mut comment = SentiComment {
                            id: String::new(),
                            holder: String::new(),
                            value: String::new(),
                            sentences: Vec::new(),
                        };
                        for attr in e.attributes().flatten() {
                            let key = attr.key.as_ref();
                            let val = String::from_utf8_lossy(&attr.value).into_owned();
                            match key {
                                b"id" => comment.id = val,
                                b"holder" => comment.holder = val,
                                b"value" => comment.value = val,
                                _ => {}
                            }
                        }
                        current_comment = Some(comment);
                    }
                    "Sentence" if in_comment => {
                        in_sentence = true;
                        let mut sent = SentiSentence {
                            id: String::new(),
                            value: String::new(),
                            text: String::new(),
                        };
                        for attr in e.attributes().flatten() {
                            let key = attr.key.as_ref();
                            let val = String::from_utf8_lossy(&attr.value).into_owned();
                            match key {
                                b"id" => sent.id = val,
                                b"value" => sent.value = val,
                                _ => {}
                            }
                        }
                        current_sentence = Some(sent);
                        current_text.clear();
                    }
                    _ => {}
                }
            }
            Ok(Event::End(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                match name.as_str() {
                    "Product" => {
                        if let Some(doc) = current_doc.take() {
                            docs.push(doc);
                        }
                        in_product = false;
                    }
                    "Opinion" | "Criticism" if in_comment => {
                        if let (Some(ref mut doc), Some(comment)) =
                            (&mut current_doc, current_comment.take())
                        {
                            doc.comments.push(comment);
                        }
                        in_comment = false;
                    }
                    "Sentence" if in_sentence => {
                        if let Some(ref mut sent) = current_sentence {
                            sent.text = current_text.trim().to_string();
                        }
                        if let (Some(ref mut comment), Some(sent)) =
                            (&mut current_comment, current_sentence.take())
                        {
                            comment.sentences.push(sent);
                        }
                        in_sentence = false;
                        current_text.clear();
                    }
                    _ => {}
                }
            }
            Ok(Event::Text(e)) if in_sentence => {
                let text = e.unescape().unwrap_or_default().into_owned();
                current_text.push_str(&text);
            }
            Ok(Event::CData(e)) if in_sentence => {
                let text = String::from_utf8_lossy(e.as_ref()).into_owned();
                current_text.push_str(&text);
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                eprintln!("SentiPersReader: XML error: {}", e);
                break;
            }
            _ => {}
        }
        buf.clear();
    }

    docs
}

/// Reads the SentiPers Persian sentiment corpus (XML format).
pub struct SentiPersReader {
    root: PathBuf,
}

impl SentiPersReader {
    /// Opens the SentiPers corpus directory.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Returns an iterator over all product documents.
    pub fn docs(&self) -> Box<dyn Iterator<Item = SentiDoc> + '_> {
        let files = collect_xml_files(&self.root);
        let mut docs: Vec<SentiDoc> = Vec::new();

        for file_path in &files {
            let content = match std::fs::read_to_string(file_path) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("SentiPersReader: cannot read {:?}: {}", file_path, e);
                    continue;
                }
            };
            docs.extend(parse_senti_docs(&content));
        }

        Box::new(docs.into_iter())
    }

    /// Returns an iterator over comments as Vec<Vec<String>> (doc → comment → sentence texts).
    pub fn comments(&self) -> Box<dyn Iterator<Item = Vec<Vec<String>>> + '_> {
        let docs: Vec<SentiDoc> = self.docs().collect();
        let result: Vec<Vec<Vec<String>>> = docs
            .into_iter()
            .map(|doc| {
                doc.comments
                    .into_iter()
                    .map(|c| c.sentences.into_iter().map(|s| s.text).collect())
                    .collect()
            })
            .collect();
        Box::new(result.into_iter())
    }
}

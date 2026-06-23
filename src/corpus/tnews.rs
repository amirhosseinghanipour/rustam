#![allow(missing_docs)]
//! Reader for the TNews Persian news corpus (XML format).

use std::{
    path::{Path, PathBuf},
};

use quick_xml::Reader;
use quick_xml::events::Event;

fn strip_html(s: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    for c in s.chars() {
        if c == '<' {
            in_tag = true;
        } else if c == '>' {
            in_tag = false;
        } else if !in_tag {
            result.push(c);
        }
    }
    result
}

fn preprocess_xml(content: &str) -> String {
    // Strip control chars 0x00-0x08, 0x0B-0x0C, 0x0E-0x1F
    let cleaned: String = content
        .chars()
        .filter(|&c| {
            let code = c as u32;
            !(code <= 0x08 || (0x0B..=0x0C).contains(&code) || (0x0E..=0x1F).contains(&code))
        })
        .collect();
    // Ensure closed </TNews> tag
    if !cleaned.contains("</TNews>") {
        format!("{}</TNews>", cleaned)
    } else {
        cleaned
    }
}

/// A document from the TNews corpus.
pub struct TNewsDoc {
    pub id: String,
    pub url: String,
    pub datetime: String,
    pub category: String,
    pub pre_title: String,
    pub title: String,
    pub post_title: String,
    pub brief: String,
    pub text: String,
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

fn parse_tnews_docs(content: &str) -> Vec<TNewsDoc> {
    let processed = preprocess_xml(content);
    let mut docs: Vec<TNewsDoc> = Vec::new();
    let mut reader = Reader::from_str(&processed);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();

    let mut in_news = false;
    let mut current_doc: Option<TNewsDoc> = None;
    let mut current_field = String::new();
    let mut current_text = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                match name.as_str() {
                    "NEWS" => {
                        in_news = true;
                        let mut doc = TNewsDoc {
                            id: String::new(),
                            url: String::new(),
                            datetime: String::new(),
                            category: String::new(),
                            pre_title: String::new(),
                            title: String::new(),
                            post_title: String::new(),
                            brief: String::new(),
                            text: String::new(),
                        };
                        for attr in e.attributes().flatten() {
                            let key = attr.key.as_ref();
                            let val = String::from_utf8_lossy(&attr.value).into_owned();
                            if key == b"id" {
                                doc.id = val;
                            }
                        }
                        current_doc = Some(doc);
                        current_field.clear();
                        current_text.clear();
                    }
                    _ if in_news => {
                        current_field = name;
                        current_text.clear();
                    }
                    _ => {}
                }
            }
            Ok(Event::End(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                match name.as_str() {
                    "NEWS" => {
                        if let Some(doc) = current_doc.take() {
                            docs.push(doc);
                        }
                        in_news = false;
                    }
                    _ if in_news => {
                        if let Some(ref mut doc) = current_doc {
                            let text = strip_html(current_text.trim());
                            match current_field.as_str() {
                                "URL" => doc.url = text,
                                "DATETIME" => doc.datetime = text,
                                "CATEGORY" => doc.category = text,
                                "PRETITLE" => doc.pre_title = text,
                                "TITLE" => doc.title = text,
                                "POSTTITLE" => doc.post_title = text,
                                "BRIEF" => doc.brief = text,
                                "TEXT" => doc.text = text,
                                _ => {}
                            }
                        }
                        current_field.clear();
                        current_text.clear();
                    }
                    _ => {}
                }
            }
            Ok(Event::Text(e)) if in_news => {
                let text = e.unescape().unwrap_or_default().into_owned();
                current_text.push_str(&text);
            }
            Ok(Event::CData(e)) if in_news => {
                let text = String::from_utf8_lossy(e.as_ref()).into_owned();
                current_text.push_str(&text);
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                eprintln!("TNewsReader: XML error: {}", e);
                break;
            }
            _ => {}
        }
        buf.clear();
    }

    docs
}

/// Reads the TNews Persian news corpus (XML format).
pub struct TNewsReader {
    root: PathBuf,
}

impl TNewsReader {
    /// Opens the TNews corpus directory.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Returns an iterator over all documents.
    pub fn docs(&self) -> Box<dyn Iterator<Item = TNewsDoc> + '_> {
        let files = collect_xml_files(&self.root);
        let mut docs: Vec<TNewsDoc> = Vec::new();

        for file_path in &files {
            let content = match std::fs::read_to_string(file_path) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("TNewsReader: cannot read {:?}: {}", file_path, e);
                    continue;
                }
            };
            docs.extend(parse_tnews_docs(&content));
        }

        Box::new(docs.into_iter())
    }

    /// Returns an iterator over all document texts.
    pub fn texts(&self) -> Box<dyn Iterator<Item = String> + '_> {
        Box::new(self.docs().map(|d| d.text))
    }
}

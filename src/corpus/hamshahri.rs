#![allow(missing_docs)]
//! Reader for the Hamshahri Persian newspaper corpus (XML format).

use std::{
    path::{Path, PathBuf},
};

use quick_xml::Reader;
use quick_xml::events::Event;

/// A document from the Hamshahri corpus.
pub struct HamshahriDoc {
    pub id: String,
    pub title: String,
    pub text: String,
    pub issue: String,
    pub date: String,
    pub categories: Vec<(String, Vec<String>)>,
}

const INVALID_FILES: &[&str] = &[
    "hamshahri.dtd",
    "HAM2-960622.xml", "HAM2-960630.xml", "HAM2-960701.xml", "HAM2-960709.xml",
    "HAM2-960710.xml", "HAM2-960711.xml", "HAM2-960817.xml", "HAM2-960818.xml",
    "HAM2-960819.xml", "HAM2-960820.xml", "HAM2-961019.xml", "HAM2-961112.xml",
    "HAM2-961113.xml", "HAM2-961114.xml", "HAM2-970414.xml", "HAM2-970415.xml",
    "HAM2-970612.xml", "HAM2-970614.xml", "HAM2-970710.xml", "HAM2-970712.xml",
    "HAM2-970713.xml", "HAM2-970717.xml", "HAM2-970719.xml", "HAM2-980317.xml",
    "HAM2-040820.xml", "HAM2-040824.xml", "HAM2-040825.xml", "HAM2-040901.xml",
    "HAM2-040917.xml", "HAM2-040918.xml", "HAM2-040920.xml", "HAM2-041025.xml",
    "HAM2-041026.xml", "HAM2-041027.xml", "HAM2-041230.xml", "HAM2-041231.xml",
    "HAM2-050101.xml", "HAM2-050102.xml", "HAM2-050223.xml", "HAM2-050224.xml",
    "HAM2-050406.xml", "HAM2-050407.xml", "HAM2-050416.xml",
];

fn collect_xml_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(collect_xml_files(&path));
            } else if path.extension().map_or(false, |e| e == "xml") {
                let fname = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("");
                if !INVALID_FILES.contains(&fname) {
                    files.push(path);
                }
            }
        }
    }
    files
}

fn parse_docs_from_content(content: &str) -> Vec<HamshahriDoc> {
    let mut docs: Vec<HamshahriDoc> = Vec::new();
    let mut reader = Reader::from_str(content);
    reader.config_mut().trim_text(false);
    let mut buf = Vec::new();

    let mut in_doc = false;
    let mut current_doc: Option<HamshahriDoc> = None;
    let mut current_field = String::new();
    let mut current_text = String::new();
    let mut current_cat_name = String::new();
    let mut current_cat_items: Vec<String> = Vec::new();
    let mut in_category = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                match name.as_str() {
                    "DOC" => {
                        in_doc = true;
                        let mut doc = HamshahriDoc {
                            id: String::new(),
                            title: String::new(),
                            text: String::new(),
                            issue: String::new(),
                            date: String::new(),
                            categories: Vec::new(),
                        };
                        for attr in e.attributes().flatten() {
                            let key = attr.key.as_ref();
                            let val = String::from_utf8_lossy(&attr.value).into_owned();
                            if key == b"id" {
                                doc.id = val;
                            }
                        }
                        current_doc = Some(doc);
                        current_field = String::new();
                        current_text = String::new();
                    }
                    "CAT" if in_doc => {
                        in_category = true;
                        current_cat_name.clear();
                        current_cat_items.clear();
                        for attr in e.attributes().flatten() {
                            let key = attr.key.as_ref();
                            let val = String::from_utf8_lossy(&attr.value).into_owned();
                            if key == b"name" {
                                current_cat_name = val;
                            }
                        }
                        current_field = "CAT".to_string();
                        current_text.clear();
                    }
                    "ITEM" if in_category => {
                        current_field = "ITEM".to_string();
                        current_text.clear();
                    }
                    _ if in_doc => {
                        current_field = name;
                        current_text.clear();
                    }
                    _ => {}
                }
            }
            Ok(Event::End(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                match name.as_str() {
                    "DOC" => {
                        if let Some(doc) = current_doc.take() {
                            docs.push(doc);
                        }
                        in_doc = false;
                    }
                    "CAT" if in_doc => {
                        if let Some(ref mut doc) = current_doc {
                            doc.categories.push((current_cat_name.clone(), current_cat_items.clone()));
                        }
                        in_category = false;
                        current_field.clear();
                    }
                    "ITEM" if in_category => {
                        current_cat_items.push(current_text.trim().to_string());
                        current_field = "CAT".to_string();
                        current_text.clear();
                    }
                    _ if in_doc => {
                        if let Some(ref mut doc) = current_doc {
                            let text = current_text.trim().to_string();
                            match current_field.as_str() {
                                "TITLE" => doc.title = text,
                                "TEXT" => doc.text = text,
                                "ISSUE" => doc.issue = text,
                                "DATE" => doc.date = text,
                                _ => {}
                            }
                        }
                        current_field.clear();
                        current_text.clear();
                    }
                    _ => {}
                }
            }
            Ok(Event::Text(e)) => {
                let text = e.unescape().unwrap_or_default().into_owned();
                current_text.push_str(&text);
            }
            Ok(Event::CData(e)) => {
                let text = String::from_utf8_lossy(e.as_ref()).into_owned();
                current_text.push_str(&text);
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                eprintln!("HamshahriReader: XML error: {}", e);
                break;
            }
            _ => {}
        }
        buf.clear();
    }

    docs
}

/// Reads the Hamshahri Persian newspaper corpus (XML format).
pub struct HamshahriReader {
    root: PathBuf,
}

impl HamshahriReader {
    /// Opens the Hamshahri corpus directory.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Returns an iterator over all documents.
    pub fn docs(&self) -> Box<dyn Iterator<Item = HamshahriDoc> + '_> {
        let files = collect_xml_files(&self.root);
        let mut docs: Vec<HamshahriDoc> = Vec::new();

        for file_path in &files {
            let content = match std::fs::read_to_string(file_path) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("HamshahriReader: cannot read {:?}: {}", file_path, e);
                    continue;
                }
            };
            docs.extend(parse_docs_from_content(&content));
        }

        Box::new(docs.into_iter())
    }

    /// Returns an iterator over all document texts.
    pub fn texts(&self) -> Box<dyn Iterator<Item = String> + '_> {
        Box::new(self.docs().map(|d| d.text))
    }
}

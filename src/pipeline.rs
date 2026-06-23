//! High-level pipeline that wires POSTagger + Lemmatizer + DependencyParser.
//!
//! Enabled by the `dep-parsing` Cargo feature.  Provides a high-level
//! `DependencyParser(tagger=tagger, lemmatizer=lemmatizer)` interface so
//! callers can parse raw token lists without manually building CoNLL-U structs.
//!
//! # Example
//!
//! ```no_run
//! #[cfg(feature = "dep-parsing")]
//! {
//!     use rustam::pipeline::PipelineParser;
//!     use crftag::POSTagger;
//!     use rustam::Lemmatizer;
//!
//!     let mut tagger = POSTagger::new();
//!     tagger.load_model("pos_tagger.model").unwrap();
//!
//!     let mut parser = PipelineParser::new(tagger, Lemmatizer::new());
//!     parser.load_model("dep.model").unwrap();
//!
//!     let result = parser.parse(&["من", "کتاب", "می\u{200C}خوانم", "."]).unwrap();
//!     for (i, word) in result.words.iter().enumerate() {
//!         println!("{}\t{}\t{}", word, result.heads[i], result.deprels[i]);
//!     }
//! }
//! ```

#[cfg(feature = "dep-parsing")]
pub use arc_eager::{ConlluToken as PipelineToken, ParseResult};

/// A high-level dependency parser that accepts raw token lists.
///
/// Internally it:
/// 1. POS-tags the tokens with [`crftag::POSTagger`].
/// 2. Lemmatizes each token using [`crate::Lemmatizer`].
/// 3. Builds [`PipelineToken`] structs and runs the arc-eager parser.
///
/// Enabled by the `dep-parsing` Cargo feature.
#[cfg(feature = "dep-parsing")]
pub struct PipelineParser {
    tagger: crftag::POSTagger,
    lemmatizer: crate::lemmatizer::Lemmatizer,
    parser: arc_eager::DependencyParser,
}

#[cfg(feature = "dep-parsing")]
impl PipelineParser {
    /// Creates a pipeline.  Call [`load_model`](Self::load_model) before parsing.
    pub fn new(tagger: crftag::POSTagger, lemmatizer: crate::lemmatizer::Lemmatizer) -> Self {
        Self { tagger, lemmatizer, parser: arc_eager::DependencyParser::new() }
    }

    /// Loads the dependency parser model from `path`.
    pub fn load_model(&mut self, path: impl AsRef<std::path::Path>) -> arc_eager::Result<()> {
        self.parser.load_model(path)
    }

    /// Returns `true` if the dependency model is loaded.
    pub fn is_loaded(&self) -> bool {
        self.parser.is_loaded()
    }

    /// Parses a single tokenised sentence.
    ///
    /// Returns an error if the POS tagger or dependency parser model is not loaded.
    pub fn parse(&self, words: &[&str]) -> Result<ParseResult, Box<dyn std::error::Error>> {
        let tagged = self.tagger.tag(words)?;
        let tokens = self.build_conllu(&tagged);
        Ok(self.parser.parse(&tokens)?)
    }

    /// Parses multiple tokenised sentences.
    pub fn parse_sents(
        &self,
        sentences: &[Vec<&str>],
    ) -> Result<Vec<ParseResult>, Box<dyn std::error::Error>> {
        sentences.iter().map(|s| self.parse(s)).collect()
    }

    fn build_conllu(&self, tagged: &[(&str, String)]) -> Vec<arc_eager::ConlluToken> {
        tagged
            .iter()
            .enumerate()
            .map(|(i, (word, pos))| {
                let lemma = self.lemmatizer.lemmatize(word, pos);
                arc_eager::ConlluToken {
                    id: i + 1,
                    form: word.to_string(),
                    lemma,
                    upos: pos.clone(),
                    xpos: pos.clone(),
                    feats: "_".to_string(),
                    head: 0,
                    deprel: "_".to_string(),
                }
            })
            .collect()
    }
}

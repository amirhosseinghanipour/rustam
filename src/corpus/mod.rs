//! Corpus readers for Persian annotated corpora.
//!
//! All readers implement the [`CorpusReader`] trait and yield sentences
//! lazily from disk.  The corpora themselves are **not** bundled with rustam —
//! you must download them separately and supply the file/directory path.
//!
//! Available readers:
//! - [`BijankhanReader`] — Bijankhan POS-tagged corpus (2.6M words)
//! - [`ArmanReader`] — Arman NER corpus (IOB format)
//! - [`DadeganReader`] — Dadegan Persian Dependency Treebank (CoNLL)
//! - [`UniversalDadeganReader`] — Universal Dadegan (CoNLL-U)
//! - [`PeykareReader`] — Peykare corpus (Windows-1256 encoded)
//! - [`TreebankReader`] — Persian Treebank (XML)
//! - [`NerReader`] — Persian NER corpus (BIO/IOB)
//! - [`VerbValencyReader`] — Persian Verb Valency corpus (TSV)
//! - [`HamshahriReader`] — Hamshahri newspaper corpus (XML)
//! - [`MirasTextReader`] — MirasText corpus
//! - [`MizanReader`] — Mizan parallel Persian-English corpus
//! - [`NaabReader`] — Naab Persian text corpus
//! - [`PnSummaryReader`] — PN Summary news summarization corpus
//! - [`PersianPlainTextReader`] — Raw Persian plain-text files
//! - [`PersicaReader`] — Persica Persian news corpus
//! - [`QuranReader`] — Quran corpus (Buckwalter transliteration)
//! - [`SentiPersReader`] — SentiPers Persian sentiment corpus
//! - [`TNewsReader`] — TNews Persian news corpus
//! - [`WikipediaReader`] — Wikipedia pre-extracted text
//! - [`FaSpellReader`] — FaSpell spell-checking corpus
//! - [`DegarbayanReader`] — Degarbayan paraphrase corpus

pub mod arman;
pub mod bijankhan;
pub mod dadegan;
pub mod degarbayan;
pub mod faspell;
pub mod hamshahri;
pub mod mirastext;
pub mod mizan;
pub mod naab;
pub mod ner;
pub mod persian_plain_text;
pub mod persica;
pub mod peykare;
pub mod pn_summary;
pub mod quran;
pub mod sentipers;
pub mod tnews;
pub mod treebank;
pub mod universal_dadegan;
pub mod verbvalency;
pub mod wikipedia;

pub use arman::ArmanReader;
pub use bijankhan::BijankhanReader;
pub use dadegan::{DadeganMode, DadeganReader};
pub use degarbayan::{DegarbayanReader, ParaphrasePair};
pub use faspell::{FaSpellReader, SpellEntry};
pub use hamshahri::{HamshahriDoc, HamshahriReader};
pub use mirastext::{MirasDoc, MirasTextReader};
pub use mizan::MizanReader;
pub use naab::NaabReader;
pub use ner::NerReader;
pub use persian_plain_text::PersianPlainTextReader;
pub use persica::{PersicaDoc, PersicaReader};
pub use peykare::PeykareReader;
pub use pn_summary::{PnSummaryDoc, PnSummaryReader};
pub use quran::{QuranPart, QuranReader};
pub use sentipers::{SentiComment, SentiDoc, SentiPersReader, SentiSentence};
pub use tnews::{TNewsDoc, TNewsReader};
pub use treebank::TreebankReader;
pub use universal_dadegan::{UniversalDadeganMode, UniversalDadeganReader};
pub use verbvalency::{Verb, VerbValencyReader};
pub use wikipedia::{WikiDoc, WikipediaReader};

use crate::types::TaggedSentence;

/// Common interface for all corpus readers.
pub trait CorpusReader {
    /// Returns an iterator over sentences.
    ///
    /// Each sentence is a `Vec<(token, tag)>` pair.
    fn sents(&self) -> Box<dyn Iterator<Item = TaggedSentence> + '_>;

    /// Returns all tokens in the corpus, in order.
    fn words(&self) -> Box<dyn Iterator<Item = String> + '_> {
        Box::new(self.sents().flat_map(|s| s.into_iter().map(|(w, _)| w)))
    }

    /// Returns all (token, tag) pairs in the corpus.
    fn tagged_words(&self) -> Box<dyn Iterator<Item = (String, String)> + '_> {
        Box::new(self.sents().flat_map(|s| s.into_iter()))
    }
}

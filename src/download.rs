//! Hugging Face Hub model downloader.
//!
//! Downloads pre-trained Persian NLP models from the
//! [roshan-research](https://huggingface.co/roshan-research) HF Hub organisation
//! and returns their local cache path.
//!
//! Enabled by the `hf-hub` Cargo feature.
//!
//! # Example
//!
//! ```ignore
//! use rustam::download::{download, Model};
//!
//! let pos_path = download(Model::PosTagger).unwrap();
//! let ft_path  = download(Model::FastText).unwrap();
//! ```

#[cfg(feature = "hf-hub")]
use std::path::PathBuf;

#[cfg(feature = "hf-hub")]
use crate::error::{Error, Result};

/// Pre-trained model identifiers understood by [`download`].
pub enum Model {
    /// CRF POS tagger model (`pos_tagger.model`)
    PosTagger,
    /// CRF chunker model (`chunker.model`)
    Chunker,
    /// CRF NER model (`ner.model`)
    Ner,
    /// Arc-eager dependency parser model (`dep.model`)
    DepParser,
    /// FastText skip-gram 300-d Persian embeddings (`fasttext_skipgram_300.bin`)
    FastText,
    /// Word2Vec Persian embeddings (`word2vec.bin`)
    Word2Vec,
}

/// Downloads `model` from HF Hub and returns its local cache path.
///
/// Files are cached in the standard HF Hub cache directory and only
/// re-downloaded when the remote copy changes.
///
/// Requires the `hf-hub` Cargo feature.
#[cfg(feature = "hf-hub")]
pub fn download(model: Model) -> Result<PathBuf> {
    use hf_hub::api::sync::Api;

    let (repo_id, filename): (&str, &str) = match model {
        Model::PosTagger => ("amirhosseinghanipour/rustam-postagger",        "pos_tagger.model"),
        Model::Chunker   => ("amirhosseinghanipour/rustam-chunker",          "chunker.model"),
        Model::Ner       => ("amirhosseinghanipour/rustam-ner",              "ner.model"),
        Model::DepParser => ("amirhosseinghanipour/rustam-dependency-parser", "dep.model"),
        Model::FastText  => ("amirhosseinghanipour/rustam-word-embedding",    "fasttext_skipgram_300.bin"),
        Model::Word2Vec  => ("amirhosseinghanipour/rustam-word-embedding",    "word2vec.bin"),
    };

    let api = Api::new().map_err(|e| Error::Parse(e.to_string()))?;
    api.model(repo_id.to_string())
        .get(filename)
        .map_err(|e| Error::Parse(format!("HF Hub download failed: {e}")))
}

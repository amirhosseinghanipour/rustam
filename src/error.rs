use thiserror::Error;

/// All errors that can occur in the rustam library.
#[derive(Debug, Error)]
pub enum Error {
    #[error("verb entry has invalid format (expected 'past#present'): {0}")]
    /// A verb string did not contain the expected `past#present` separator.
    InvalidVerbFormat(String),

    #[error("failed to compile regex pattern '{pattern}': {source}")]
    /// A regular expression failed to compile.
    RegexCompile {
        /// The pattern string that failed.
        pattern: String,
        /// The underlying regex error.
        source: fancy_regex::Error,
    },
}

/// Convenience alias for `Result<T, Error>` used throughout this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// A single word token.
pub type Token = String;

/// A POS tag string (e.g. "NOUN", "VERB", "ADJ").
pub type Tag = String;

/// A token paired with its POS tag.
pub type TaggedToken = (Token, Tag);

/// An ordered list of tokens representing one sentence.
pub type Sentence = Vec<Token>;

/// A sentence where every token carries its POS tag.
pub type TaggedSentence = Vec<TaggedToken>;

/// An IOB tag for chunking (e.g. "B-NP", "I-VP", "O").
pub type IobTag = String;

/// A token with its POS tag and IOB chunk tag.
pub type ChunkedToken = (Token, Tag, IobTag);

/// A full sentence of chunked tokens.
pub type ChunkedSentence = Vec<ChunkedToken>;

/// A word entry from the lexicon: (frequency, part-of-speech tags).
#[derive(Debug, Clone)]
pub struct WordEntry {
    /// Corpus frequency of this word form.
    pub frequency: u64,
    /// All POS tags observed for this word form (e.g. `["N", "AJ"]`).
    pub pos_tags: Vec<String>,
}

/// Parsed verb roots: past stem and present stem.
#[derive(Debug, Clone)]
pub struct VerbRoots {
    /// بن ماضی — past root (e.g. "دید")
    pub past: String,
    /// بن مضارع — present root (e.g. "بین")
    pub present: String,
}

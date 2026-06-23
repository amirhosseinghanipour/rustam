# rustam

A full Persian NLP pipeline for Rust — normalization, tokenization, stemming, lemmatization, conjugation, informal-to-formal conversion, spell correction, and more.

## Features

- **Normalization** — character translation, diacritics removal, spacing fixes, Unicode ligature replacement
- **Informal normalization** — colloquial-to-formal Persian conversion with multi-candidate output
- **Sentence tokenization** — boundary detection using Persian and Latin punctuation patterns
- **Word tokenization** — whitespace/punctuation splitting with optional multi-part verb joining
- **Stemming** — rule-based suffix stripping (longest-match, lexicon-aware)
- **Lemmatization** — dictionary lookup + full conjugation table fallback
- **Conjugation** — generates every tense/mood/voice form for a Persian verb root pair
- **Token splitting** — compound token decomposition using the word lexicon
- **Spell correction** — frequency-ranked correction from the embedded lexicon
- **Corpus readers** — Bijankhan, Arman, Peykare, Hamshahri, NAAB, and more
- **Embedding traits** — `WordEmbedding` / `SentenceEmbedding` interfaces for pluggable backends

## Quick start

```toml
[dependencies]
rustam = "0.1"
```

```rust
use rustam::{Normalizer, SentenceTokenizer, WordTokenizer, Stemmer, Lemmatizer};

let text = "اِعلاممممم کَرد : « زمین لرزه ای به بُزرگیِ 6 دهم ریشتر ...»";

let norm = Normalizer::new();
let normalized = norm.normalize(text);
// → "اعلام کرد: «زمین‌لرزه‌ای به بزرگی ۶ دهم ریشتر …»"

let stemmer = Stemmer::new();
assert_eq!(stemmer.stem("کتاب‌ها"), "کتاب");

let lem = Lemmatizer::new();
assert_eq!(lem.lemmatize("می‌روم", ""), "رفت#رو");
```

### Informal normalization

```rust
use rustam::InformalNormalizer;

let n = InformalNormalizer::new();
let candidates = n.normalized_word("واسه");
// → ["برای", "واسه"]  (informal → formal candidates)
```

### Conjugation

```rust
use rustam::Conjugation;

let c = Conjugation;
let forms = c.perfective_past("دید");
// → ["دیدم", "دیدی", "دید", "دیدیم", "دیدید", "دیدند"]
```

## Optional features

| Feature | Enables |
|---------|---------|
| `pos` | POS tagger and chunker (`crftag`) |
| `ner` | Named entity recognizer (`nerrs`) |
| `dep-parsing` | Arc-eager dependency parser (`arc-eager`) |
| `hf-hub` | Model download from Hugging Face Hub |
| `full` | All of the above |

## Pre-trained models

The `hf-hub` feature downloads pre-trained Persian NLP models (POS tagger, chunker, NER, dependency parser, word embeddings) from Hugging Face Hub. Pre-trained weights are provided by the [roshan-research](https://huggingface.co/roshan-research) organization and are used here with gratitude.

## License

MIT

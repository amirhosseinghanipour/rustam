use std::collections::HashMap;

/// Builds a character-to-character translation map from two parallel strings.
///
/// Characters at the same position in `src` and `dst` are paired.
pub fn make_trans(src: &str, dst: &str) -> HashMap<char, char> {
    src.chars().zip(dst.chars()).collect()
}

/// Applies a character translation map to a string.
pub fn translate(text: &str, table: &HashMap<char, char>) -> String {
    text.chars()
        .map(|c| *table.get(&c).unwrap_or(&c))
        .collect()
}

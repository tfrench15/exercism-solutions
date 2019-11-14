use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    let v: Vec<&str> = words.split(|c: char| c.is_ascii_whitespace() || c == ',').collect();

    for word in v {
        if word.len() == 0 {
            continue
        }

        let clean_word = word.trim_matches(|c: char| c.is_ascii_punctuation()).to_string();
        let counter = counts.entry(clean_word.to_ascii_lowercase()).or_insert(0);
        *counter += 1;
    }

    counts
}

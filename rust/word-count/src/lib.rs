use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    let v: Vec<&str> = words.split(|c: char| c.is_ascii_whitespace() || c == ',').collect();
    let mut normalized_words = Vec::new();

    for word in v {
        let normalized_word = word.trim_matches(|c: char| c.is_ascii_punctuation()).to_ascii_lowercase();

        if normalized_word.len() == 0 {
            continue
        }

        normalized_words.push(normalized_word);
    }

    for word in normalized_words {
        let counter = counts.entry(word).or_insert(0);
        *counter += 1;
    }

    counts
}

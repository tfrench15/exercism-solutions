use std::collections::HashMap;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut alphabet = HashMap::new();

    for ch in sentence.to_lowercase().chars() {
        if !ch.is_ascii_alphabetic() {
            continue
        } else {
            let counter = alphabet.entry(ch).or_insert(0);
            *counter += 1;
        }
    }

    alphabet.len() == 26
}

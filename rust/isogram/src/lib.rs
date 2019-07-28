use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    if candidate.len() == 0 {
        return true
    }

    let mut counts = HashMap::new();
    for ch in candidate.to_lowercase().chars() {
        if !ch.is_alphabetic() {
            continue
        }
        let counter = counts.entry(ch).or_insert(0);
        *counter += 1;
    }

    for value in counts.values() {
        if *value > 1 {
            return false
        }
    }

    true
}

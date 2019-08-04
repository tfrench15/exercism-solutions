use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();
    let mut word_map = HashMap::new();

    for ch in word.chars().map(|ch| ch.to_ascii_lowercase()) {
        let count = word_map.entry(ch).or_insert(0);
        *count += 1;
    }

    for candidate in possible_anagrams.iter() {
        if candidate.to_lowercase() == word.to_lowercase() {
            continue
        } else {
            let mut candidate_map = HashMap::new();
            for ch in candidate.chars().map(|ch| ch.to_ascii_lowercase()) {
                let count = candidate_map.entry(ch).or_insert(0);
                *count += 1;
            }
            if word_map == candidate_map {
                anagrams.insert(*candidate);
            }
        }   
    }

    anagrams
}

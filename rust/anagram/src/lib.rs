use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();
    let mut word_map = HashMap::new();

    for ch in word.chars() {
        for lc in ch.to_lowercase() {
            let count = word_map.entry(lc).or_insert(0);
            *count += 1;
        }   
    }

    for candidate in possible_anagrams.iter() {
        if is_same_word(word, candidate) {
            continue
        } else {
            let mut candidate_map = HashMap::new();
            for ch in candidate.chars() {
                for lc in ch.to_lowercase() {
                    let count = candidate_map.entry(lc).or_insert(0);
                    *count += 1;
                }
            }
            if word_map == candidate_map {
            anagrams.insert(*candidate);
            }
        } 
    }

    anagrams
}

fn is_same_word(word: &str, candidate: &str) -> bool {
    for pair in word.chars().zip(candidate.chars()) {
        for lc in pair.0.to_lowercase().zip(pair.1.to_lowercase()) {
            if lc.0 != lc.1 {
                return false
            }
        }
    }

    true
}
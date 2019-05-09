use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut count: usize = 0;

    if !is_valid_char(nucleotide) {
        return Err(nucleotide)
    }

    for ch in dna.chars() {
        if !is_valid_char(ch) {
            return Err(ch)
        } 
        if ch == nucleotide {
            count += 1;
        }
    }
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map: HashMap<char, usize> = HashMap::new();
    for ch in dna.chars() {
        if !is_valid_char(ch) {
            return Err(ch)
        } else {
            *map.entry(ch).or_insert(0) += 1;
        }
    }
    Ok(map)
}

fn is_valid_char(ch: char) -> bool {
    let valid_chars = vec!['A', 'C', 'G', 'T'];
    valid_chars.contains(&ch)
}


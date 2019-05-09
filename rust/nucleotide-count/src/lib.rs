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
    let mut map: HashMap<char, usize> = 
        [('A', 0),
         ('C', 0),
         ('G', 0),
         ('T', 0)]
         .iter().cloned().collect();
    for ch in dna.chars() {
        if !is_valid_char(ch) {
            return Err(ch)
        } else {
            let counter = map.entry(ch).or_insert(0);
            *counter += 1;
        }
    }
    Ok(map)
}

fn is_valid_char(ch: char) -> bool {
    let valid_chars = vec!['A', 'C', 'G', 'T'];
    valid_chars.contains(&ch)
}


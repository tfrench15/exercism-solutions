use std::collections::HashMap;

pub fn rotate(input: &str, key: i8) -> String {
    const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";

    

    let mut upper_char_to_idx = HashMap::new();
    let mut upper_idx_to_char = HashMap::new();

    let mut lower_char_to_idx = HashMap::new();
    let mut lower_idx_to_char = HashMap::new();

    for (idx, ch) in UPPERCASE.chars().enumerate() {
        upper_idx_to_char.insert(idx as i8, ch);
        upper_char_to_idx.insert(ch, idx as i8);
    }

    for (idx, ch) in LOWERCASE.chars().enumerate() {
        lower_idx_to_char.insert(idx as i8, ch);
        lower_char_to_idx.insert(ch, idx as i8);
    }

    input
        .chars()
        .map(|ch| {
            if ch.is_ascii_uppercase() {
                match upper_char_to_idx.get(&ch) {
                    None => { ch },
                    Some(v) => {
                        let mut rotated_idx = v + key;
                        if rotated_idx < 0 {
                            while rotated_idx < 0 {
                                rotated_idx += 26;
                            }
                        }
                        *upper_idx_to_char.get(&(rotated_idx % 26)).unwrap()
                    }
                }
            } else if ch.is_ascii_lowercase() {
                match lower_char_to_idx.get(&ch) {
                    None => { ch },
                    Some(v) => { 
                        let mut rotated_idx = v + key;
                        if rotated_idx < 0 {
                            while rotated_idx < 0 {
                                rotated_idx += 26;
                            }
                        }
                        *lower_idx_to_char.get(&(rotated_idx % 26)).unwrap()
                    }
                }
            } else { 
                ch
            }
        })
        .collect()
}
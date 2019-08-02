pub fn encrypt(input: &str) -> String {
    if input.len() == 0 {
        return "".to_string()
    }

    let normalized: String = input
        .chars()
        .filter(|ch| ch.is_ascii_alphabetic())
        .map(|ch| ch.to_ascii_lowercase())
        .collect();

    let cols = columns(normalized.len());

    let chars: Vec<char> = normalized
        .chars()
        .collect();
    
    let mut vec_of_words: Vec<String> = Vec::new();

    for chunk in chars.chunks(cols) {
        let word: String = chunk
            .iter()
            .collect();
        
        vec_of_words.push(word);
    }

    let encrypted_words = apply_cipher(&vec_of_words);

    encrypted_words.join(" ")
}


fn columns(length: usize) -> usize {
    let mut i: usize = 1;
    while i.pow(2) < length {
        i += 1;
    }

    i
}

fn apply_cipher(words: &[String]) -> Vec<String> {
    let mut idx: usize = 0;
    let mut encrypted_words = Vec::new();

    while idx < words[0].len() {
        let mut encrypted_word = String::new();
        for j in 0..words.len() {
            match words[j].get(idx..idx+1) {
                None => { 
                    encrypted_word.push_str(" "); 
                },
                Some(letter) => { 
                    encrypted_word.push_str(letter);
                }
            }
        }
        encrypted_words.push(encrypted_word);
        idx += 1;
    }

    encrypted_words
}
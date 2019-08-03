pub fn encrypt(input: &str) -> String {
    if input.len() == 0 {
        return "".to_string()
    }

    let normalized: String = input
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric() && !ch.is_ascii_whitespace())
        .map(|ch| ch.to_ascii_lowercase())
        .collect();

    let rectangle = perfect_rectangle(normalized.len());

    let chars: Vec<char> = normalized
        .chars()
        .collect();

    let mut vec_of_words: Vec<String> = Vec::new();

    for chunk in chars.chunks(rectangle.0) {
        let word: String = chunk
            .iter()
            .collect();
        
        vec_of_words.push(word);
    }

    let encrypted_words = apply_cipher(&vec_of_words);

    encrypted_words.join(" ")
}

fn perfect_rectangle(length: usize) -> (usize, usize, usize) {
    let mut columns: usize = 1;
    let mut rows: usize = 1;

    while (columns * rows) < length {
        if columns == rows {
            columns += 1;
        } else {
            rows += 1;
        }
    }

    (columns, rows, columns * rows)
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
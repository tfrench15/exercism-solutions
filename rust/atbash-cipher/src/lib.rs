use std::collections::HashMap;

struct Cipher {
    mappings: HashMap<char, char>
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let key = Cipher::new();

    let normalized: Vec<char> = plain
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .map(|ch| ch.to_ascii_lowercase())
        .map(|ch| {
            match key.mappings.get(&ch) {
                None => { ch },
                Some(v) => { *v },
            }
        })
        .collect();

    let mut encrypted_phrase: Vec<String> = Vec::new();

    for char_set in normalized.chunks(5) {
        let encrypted_word: String = char_set
            .iter()
            .collect();

        encrypted_phrase.push(encrypted_word);
    }

    encrypted_phrase.join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let key = Cipher::new();

    let decoded_phrase: String = cipher
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .map(|ch| {
            match key.mappings.get(&ch) {
                None => { ch },
                Some(v) => { *v },
            }
        })
        .collect();

    decoded_phrase
}

impl Cipher {
    fn new() -> Self {
        let mut mappings = HashMap::new();
        let alphabet = vec![
            'a',
            'b',
            'c',
            'd',
            'e',
            'f',
            'g',
            'h',
            'i',
            'j',
            'k',
            'l',
            'm',
            'n',
            'o',
            'p',
            'q',
            'r',
            's',
            't',
            'u',
            'v',
            'w',
            'x',
            'y',
            'z'
        ];

        for pair in alphabet.iter().zip(alphabet.iter().rev()) {
            mappings.insert(*pair.0, *pair.1);
        }

        Cipher { mappings: mappings }
    }
}
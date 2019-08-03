use std::collections::HashMap;

struct Cipher {
    mappings: HashMap<char, char>
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let cipher = Cipher::new();

    plain
        .chars()
        .filter()

    String::new()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let cipher = Cipher::new();

    String::new()
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

pub fn abbreviate(phrase: &str) -> String {
    if phrase.len() == 0 {
        return String::from(phrase)
    }

    let mut acronym = String::new();

    for word in phrase.split_ascii_whitespace() {
        acronym.push(first_char(word));

        let mut is_punc = word.chars().nth(0).unwrap().is_ascii_punctuation();
        let mut is_lower = word.chars().nth(0).unwrap().is_lowercase();
        for ch in word.chars().skip(1) {
            if ch.is_ascii_punctuation() {
                is_punc = true;
                continue;
            }

            if is_punc && !ch.is_ascii_punctuation() {
                acronym.push(ch.to_ascii_uppercase());
                is_punc = false;
            }

            if ch.is_lowercase() {
                is_lower = true;
                continue;
            }

            if is_lower && ch.is_uppercase() {
                acronym.push(ch.to_ascii_uppercase());
                is_lower = false;
            }
        }
    }

    acronym
}

fn first_char(word: &str) -> char {
    word
        .chars()
        .nth(0)
        .unwrap()
        .to_ascii_uppercase()
}


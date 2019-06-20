pub fn abbreviate(phrase: &str) -> String {
    if phrase.len() == 0 {
        return String::from(phrase)
    }

    let mut acronym: String = phrase
        .split_ascii_whitespace() // ["carbon", "metal-oxide", "HyperText"]
        .flat_map(|elem| elem.split(char::is_ascii_punctuation)) // ["carbon", ["metal", "oxide", "hyperText"]]
        .chars()
        .nth(0)
        .unwrap()
        .to_ascii_uppercase()
        .collect();

    acronym  
}


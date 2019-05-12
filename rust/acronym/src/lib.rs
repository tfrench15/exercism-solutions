pub fn abbreviate(phrase: &str) -> String {
    if phrase.len() == 0 {
        return String::from(phrase)
    }

    let mut acronym = String::new();
    for word in phrase.split_ascii_whitespace() {
        acronym.push(word.chars().next().unwrap().to_ascii_uppercase());
    }
    acronym
}

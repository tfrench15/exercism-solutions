pub fn encrypt(input: &str) -> String {
    let normalized: String = input
        .chars()
        .filter(|ch| ch.is_ascii_alphabetic())
        .map(|ch| ch.to_ascii_lowercase())
        .collect();
    
    normalized
}

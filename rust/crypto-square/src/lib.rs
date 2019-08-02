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
}

fn columns(length: usize) -> usize {
    let mut i: usize = 1;
    while i.pow(2) < length {
        i += 1;
    }

    i
}
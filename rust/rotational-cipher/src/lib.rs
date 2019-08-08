use std::collections::HashMap;

pub fn rotate(input: &str, key: i8) -> String {
    let uppercase_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .enumerate();

    let lowercase_alphabet = "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .enumerate();
    
    let uppercase_map: HashMap<i8, char> = uppercase_alphabet
        .iter()
        .cloned()
        .collect();
    
    let lowercase_map: HashMap<i8, char> = lowercase_alphabet
        .iter()
        .cloned()
        .collect();
    
    String::new()

}
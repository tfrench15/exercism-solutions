pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut v = Vec::new();
    let mut i = 0;
    let mut j = len;

    while j <= digits.len() {
        let s = String::from(&digits[i..j]);
        v.push(s);
        i += 1;
        j += 1;
    }
    v
}

pub fn is_armstrong_number(num: u32) -> bool {
    let s = num.to_string();
    let exp = s.len() as u32;
    let mut armstrong = 0;

    for ch in s.chars() {
        let dig = ch.to_digit(10).expect("not a valid digit");
        armstrong += dig.pow(exp);
    }
    if armstrong != num {
        return false
    }
    true
}

#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    // Case I: span is too long
    if span > string_digits.len() {
        return Err(Error::SpanTooLong)
    }

    // Case II: span is 0
    if span == 0 {
        return Ok(1)
    }

    // Case III: contains chars
    for ch in string_digits.chars() {
        if ch.is_alphabetic() {
            return Err(Error::InvalidDigit(ch))
        }
    }

    // Case IV: loop
    let mut max = 0;
    let digits: Vec<u64> = string_digits
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as u64)
        .collect();

    for window in digits.windows(span) {
        let product = window.iter().product();
        if product > max {
            max = product;
        }
    } 

    Ok(max)
}

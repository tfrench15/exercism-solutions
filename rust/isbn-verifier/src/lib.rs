/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let filtered_isbn: String = isbn.chars().filter(|ch| {
        ch.is_digit(10) || *ch == 'X'
    }).collect();

    match filtered_isbn.len() {
        10 => { 
            if has_valid_check_digit(&filtered_isbn) {
                let mut digits: Vec<u32> = Vec::new();
                filtered_isbn
                    .chars()
                    .for_each(|ch| {
                        if ch == 'X' {
                            digits.push(10);
                        } else {
                            digits.push(ch.to_digit(10).unwrap());
                        }
                    });

                let mut sum = 0;
                let rev = (1..=10).rev();
                let sum_iter = digits.iter().zip(rev);
                for pair in sum_iter {
                    sum += pair.0 * pair.1;
                }

                return sum % 11 == 0
            } else {
                return false
            }
        },
        _ => { return false }
    }
}

fn has_valid_check_digit(isbn: &str) -> bool {
    match isbn.find('X') {
        Some(idx) => { 
            if idx != 9 {
                return false
            } else {
                return true
            }
        },
        None => { return true }
    }
}
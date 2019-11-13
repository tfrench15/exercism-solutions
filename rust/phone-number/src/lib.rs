pub fn number(user_number: &str) -> Option<String> {
    let normalized_number = get_normalized_number(user_number);

    match normalized_number {
        Some(num) => { 
            if is_valid_area_code(&num) && is_valid_exchange_code(&num) {
                return Some(num)
            } else {
                return None
            }
        },
        None => { None }
    }
}

fn get_normalized_number(user_number: &str) -> Option<String> {
    let mut normalized_number: String = user_number
        .chars()
        .filter(|num| num.is_ascii_digit())
        .collect();

    match normalized_number.len() {
        10 => { Some(normalized_number) },
        11 => { 
            if normalized_number.chars().next().unwrap() == '1' {
                normalized_number = normalized_number.chars().skip(1).collect();
                return Some(normalized_number)
            } else {
                return None
            }
        },
        _ => { None }
    }
}

fn is_valid_area_code(user_number: &str) -> bool {
    let area_code: u32 = user_number
        .chars()
        .nth(0)
        .unwrap()
        .to_digit(10)
        .unwrap();

    if area_code > 1 {
        return true
    }

    false
}

fn is_valid_exchange_code(user_number: &str) -> bool {
    let exchange_code: u32 = user_number
        .chars()
        .nth(3)
        .unwrap()
        .to_digit(10)
        .unwrap();

    if exchange_code > 1 {
        return true
    }

    false
}
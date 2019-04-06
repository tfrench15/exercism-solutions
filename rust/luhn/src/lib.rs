use std::collections::HashMap;

pub fn is_valid(code: &str) -> bool {
    let trimmed_code = code.trim();
    if trimmed_code.len() <= 1 {
        return false
    }

    let double_map: HashMap<char, char> =
        [('0', '0'),
         ('1', '2'),
         ('2', '4'),
         ('3', '6'), 
         ('4', '8'),
         ('5', '1'),
         ('6', '3'),
         ('7', '5'),
         ('8', '7'),
         ('9', '9')]
         .iter().cloned().collect();
    
    const RADIX: u32 = 10;

    let mut sum = 0;
    let mut result = String::new();
    let mut double = false;

    for dig in trimmed_code.chars().rev() {
        if !double_map.contains_key(&dig) {
            if dig == ' ' {
                continue
            } else {
                return false
            }
        }
        if double {
            result.push(double_map[&dig]);
            double = false;
        } else {
            result.push(dig);
            double = true;
        }
    }

    for ch in result.chars() {
        sum += ch.to_digit(RADIX).unwrap();
    }

    if sum % 10 == 0 {
        return true
    }
    false
}

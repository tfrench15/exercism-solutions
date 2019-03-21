use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    if string.len() == 0 {
        return true
    }

    let pairs: HashMap<char, char> = 
        [('{', '}'),
         ('[', ']'),
         ('(', ')')]
         .iter().cloned().collect();

    let mut stack = Vec::new();

    for ch in string.chars() {
        if pairs.contains_key(&ch) {
            stack.push(pairs[&ch]);
            continue;
        }

        if is_closing_bracket(ch) {
            if !stack.contains(&ch) {
                return false
            }
            if stack.len() > 0 {
                let expected = stack[stack.len()-1];
                if expected == ch {
                    stack.pop();
                    continue;
                }
            }
        }
    }
    stack.len() == 0
}

fn is_closing_bracket(ch: char) -> bool {
    if ch == ')' || ch == '}' || ch == ']' {
        return true
    }
    false
}

pub fn reply(message: &str) -> &str {
    let trimmed_message = message.trim();

    if is_empty(trimmed_message) {
        return "Fine. Be that way!"
    }
    
    if is_question(trimmed_message) {
        if is_shouting(trimmed_message) {
            return "Calm down, I know what I'm doing!"
        }
        return "Sure."
    }

    if is_shouting(trimmed_message) {
        return "Whoa, chill out!"
    }
    
    "Whatever."
}

fn is_empty(message: &str) -> bool {
    if message == "" {
        return true
    }
    false
}

fn is_question(message: &str) -> bool {
    if message.ends_with("?") {
        return true
    }
    false
}

fn is_shouting(message: &str) -> bool {
    if !has_letters(message) {
        return false
    }

    let shout = message.to_uppercase();
    let shout = shout.as_str();

    if message == shout {
        return true
    }
    false
}


fn has_letters(message: &str) -> bool {
    for ch in message.chars() {
        if ch.is_alphabetic() {
            return true
        }
    }
    false
}

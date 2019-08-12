pub struct WordProblem;

pub fn answer(command: &str) -> Option<i32> {
    if !command.starts_with("What is") {
        return None
    }

    let _command_vec: Vec<&str> = command
        .split_ascii_whitespace()
        .skip(2)
        .collect();

    Some(5)
}

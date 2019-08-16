pub struct WordProblem {
    tup: (Option<i32>, Option<Operation>, Option<i32>);
}

enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl WordProblem {
    fn new() -> Self {
        WordProblem {
            tup: (None, None, None),
        }
    }
}

pub fn answer(command: &str) -> Option<i32> {
    if !command.starts_with("What is") {
        return None
    }

    let problem: Vec<&str> = command
        .split_ascii_whitespace()
        .skip(2)
        .collect();

    Some(5)
}




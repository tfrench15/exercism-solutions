pub struct WordProblem {
    num1: Option<i32>,
    operation: Option<&'static str>,
    num2: Option<i32>,
}

impl WordProblem {
    fn new(sentence: &[&str]) -> Option<Self> {
        let mut word_problem = WordProblem {
            num1: None,
            operation: None,
            num2: None,
        };

        match sentence.len() {
            1 => { 
                match sentence[0].parse() {
                    Ok(num) => { 
                        word_problem.num1 = Some(num);
                    },
                    Err(e) => { None },
                }

                Some(word_problem)
            },
            3 => { 
                match sentence[0].parse() {
                    Ok(num) => { 
                        word_problem.num1 = Some(num);
                    },
                    Err(e) => { None },
                },
                match sentence[1].parse() {
                    Ok(num) => { None },
                    Err(e) => { 
                        word_problem.operation = Some(sentence[1]);
                    }, 
                },
                match sentence[2].parse() {
                    Ok(num) => {
                        word_problem.num2 = Some(num);
                    },
                    Err(e) => { None },
                }
            },
        }

        Some(word_problem)
    }

    fn solve(&self) -> Option<i32> {
        match (self.num1.is_some(), self.operation.is_some(), self.num2.is_some()) {
            (true, true, true) => {
                match self.operation.unwrap() {
                    "plus" => { Some(self.num1.unwrap() + self.num2.unwrap()) },
                    "minus" => { Some(self.num1.unwrap() - self.num2.unwrap()) },
                    "multiplied" => { Some(self.num1.unwrap() * self.num2.unwrap()) },
                    "divided" => { Some(self.num1.unwrap() / self.num2.unwrap()) },
                    _ => { None },
                }
            },
            (true, false, false) => { self.num1 },
            _ => { None },
        }
    }
}

pub fn answer(command: &'static str) -> Option<i32> {
    if !command.starts_with("What is") {
        return None
    }

    let mut word_problem = WordProblem::new();

    let wp: Vec<&str> = command
        .split_ascii_whitespace()
        .skip(2)
        .filter(|word| *word != "by")
        .collect(); 

    match wp.len() {
        // the word problem comprises just one number
        1 => { 
            let mut problem = Problem::new();
            problem.num1 = Some(wp[0].parse().unwrap());

            problem.solve()
        },  

        // we have a full word problem
        3 => { 
            let mut problem = Problem::new();
            problem.num1 = Some(wp[0].parse().unwrap());
            problem.operation = Some(wp[1]);
            problem.num2 = Some(wp[2].parse().unwrap());

            problem.solve()
        },

        // there are multiple problems
        5 => { 
            let mut problem = Problem::new();
            problem.num1 = Some(wp[0].parse().unwrap());
            problem.operation = Some(wp[1]);
            problem.num2 = Some(wp[2].parse().unwrap());

            let mut second_problem = Problem::new();
            second_problem.num1 = problem.solve();
            second_problem.operation = Some(wp[3]);
            second_problem.num2 = Some(wp[4].parse().unwrap());

            second_problem.solve()
        },
        _ => { None },
    }
}
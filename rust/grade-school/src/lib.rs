use std::collections::HashMap;

pub struct School {
    roster: HashMap<String, u32>,
}

impl School {
    pub fn new() -> School {
        School {
            roster: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.roster.insert(student.to_string(), grade);
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut levels: Vec<u32> = self.roster.values()
            .map(|v| v.to_owned())
            .collect();

        levels.sort();
        levels.dedup();

        levels
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        let mut class = Vec::new();

        for key in self.roster.keys() {
            match self.roster.get(key) {
                Some(v) => {
                    if *v == grade {
                        class.push(key.to_string())
                    }
                },
                None => {
                    continue
                }
            }
        }

        if class.len() > 0 {
            class.sort();
            return Some(class)
        } 
        None
    }
}

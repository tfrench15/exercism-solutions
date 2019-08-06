use rand::{thread_rng, Rng};

pub struct Robot {
    name: String
}

const ALPHABET = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

impl Robot {
    pub fn new() -> Self {
        let mut name = String::new();
        for i in 0..5 {
            
        }

        Robot {
            name: String::new()
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self = self.new();
    }
}

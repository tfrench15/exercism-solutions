use rand::{Rng};

pub struct Robot {
    name: String
}

const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

impl Robot {
    pub fn new() -> Self {
        let mut name = String::new();
        let mut idx = name.len();
        
        while idx < 5 {
            if idx == 0 || idx == 1 {
                let letter = generate_random_letter();
                name.push(letter);
                idx += 1;
            } else {
                let number = generate_random_number();
                let s = number.to_string();
                name.push_str(&s);
                idx += 1;
            }
        }

        Robot { name: name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        let new_robot = Robot::new();
        self.name = new_robot.name.to_string();
    }
}

fn generate_random_letter() -> char {
    let mut rng = rand::thread_rng();
    let rand_num: u32 = rng.gen_range(0, 26);

    ALPHABET
        .chars()
        .nth(rand_num as usize)
        .unwrap()
}

fn generate_random_number() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0, 10)
}
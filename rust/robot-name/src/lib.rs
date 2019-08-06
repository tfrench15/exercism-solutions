pub struct Robot {
    name: String
}

impl Robot {
    pub fn new() -> Self {
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

use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut hrs = hours;
        let mut mins = minutes;

        while mins >= 60 {
            mins -= 60;
            hrs += 1;
        }
        
        while mins < 0 {
            mins += 60;
            hrs -= 1;
        }

        while hrs >= 24 {
            hrs -= 24;
        }

        while hrs < 0 {
            hrs += 24;
        }
        
        Clock {
            hours: hrs,
            minutes: mins
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
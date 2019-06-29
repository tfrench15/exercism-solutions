// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    dir: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            x: x,
            y: y,
            dir: d,
        }
    }

    pub fn turn_right(self) -> Self {
        match self.dir {
            Direction::North => { 
                Robot {
                    x: self.x,
                    y: self.y,
                    dir: Direction::East,
                }
            },
            Direction::South => {
                Robot {
                    x: self.x,
                    y: self.y,
                    dir: Direction::West,
                }
            },
            Direction::East => {
                Robot {
                    x: self.x,
                    y: self.y,
                    dir: Direction::South,
                }
            },
            Direction::West => {
                Robot {
                    x: self.x,
                    y: self.y,
                    dir: Direction::North,
                }
            }
        }
    }

    pub fn turn_left(self) -> Self {
        match self.dir {
            Direction::North => { 
                Robot {
                    x: self.x,
                    y: self.y,
                    dir: Direction::West,
                }
            },
            Direction::South => {
                Robot {
                    x: self.x,
                    y: self.y,
                    dir: Direction::East,
                }
            },
            Direction::East => {
                Robot {
                    x: self.x,
                    y: self.y,
                    dir: Direction::North,
                }
            },
            Direction::West => {
                Robot {
                    x: self.x,
                    y: self.y,
                    dir: Direction::South,
                }
            }
        }
    }

    pub fn advance(self) -> Self {
        match self.dir {
            Direction::North => { 
                Robot {
                    x: self.x,
                    y: self.y + 1,
                    dir: self.dir,
                }
            },
            Direction::South => {
                Robot {
                    x: self.x,
                    y: self.y - 1,
                    dir: self.dir,
                }
            },
            Direction::East => {
                Robot {
                    x: self.x + 1,
                    y: self.y,
                    dir: self.dir,
                }
            },
            Direction::West => {
                Robot {
                    x: self.x - 1,
                    y: self.y,
                    dir: self.dir,
                }
            }
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        unimplemented!(
            "Follow the given sequence of instructions: {}",
            instructions
        )
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.dir
    }
}

pub struct Triangle {
    side_one: u64,
    side_two: u64,
    side_three: u64,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if (sides[0] + sides[1] <= sides[2]) || (sides[0] + sides[2] <= sides[1]) || (sides[1] + sides[2] <= sides[0]) {
            return None
        }

        Some(Triangle {
            side_one: sides[0],
            side_two: sides[1],
            side_three: sides[2],
        })
    }

    pub fn is_equilateral(&self) -> bool {
        self.side_one == self.side_two && self.side_two == self.side_three
    }

    pub fn is_scalene(&self) -> bool {
        self.side_one != self.side_two && self.side_two != self.side_three && self.side_one != self.side_three
    }

    pub fn is_isosceles(&self) -> bool {
        (self.side_one == self.side_two && self.side_one != self.side_three) 
        || (self.side_one == self.side_three && self.side_one != self.side_two)
        || (self.side_two == self.side_three && self.side_two != self.side_one)
    }
}

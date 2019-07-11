#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    frames: Vec<Frame>,
}

#[derive(PartialEq)]
enum FrameStatus {
    Incomplete,
    Strike,
    Spare,
    Open,
}

struct Frame {
    status: FrameStatus,
    rolls: u16,
    roll_one: u16,
    roll_two: Option<u16>,
    roll_three: Option<u16>,
}   

impl Frame {
    fn new() -> Self {
        Frame {
            status: FrameStatus::Incomplete,
            rolls: 0,
            roll_one: 0,
            roll_two: None,
            roll_three: None,
        }
    }

    fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft)
        }

        match self.rolls {
            0 => {
                self.rolls += 1;
                self.roll_one += pins;
                if pins == 10 {
                    self.status = FrameStatus::Strike;
                }

                Ok(())
            },
            1 => { 
                if self.status == FrameStatus::Incomplete && self.roll_one + pins > 10 {
                    return Err(Error::NotEnoughPinsLeft)
                } 

                self.rolls += 1;
                self.roll_two = Some(pins);
                if self.roll_one + self.roll_two.unwrap() == 10 {
                    self.status = FrameStatus::Spare;
                } else {
                    self.status = FrameStatus::Open;
                }

                Ok(())
            }, 
            _ => { Err(Error::NotEnoughPinsLeft) },
        }
    }

    fn roll_tenth_frame(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft)
        }

        match self.rolls {
            0 => {
                self.rolls += 1;
                self.roll_one += pins;
                if self.roll_one == 10 {
                    self.status = FrameStatus::Strike;
                }

                Ok(())
            },
            1 => {
                if self.status == FrameStatus::Strike {
                    self.rolls += 1;
                    self.roll_two = Some(pins);
                    return Ok(())
                } else {
                    if self.roll_one + pins > 10 { 
                        return Err(Error::NotEnoughPinsLeft)
                    }
                    
                    self.rolls += 1;
                    self.roll_two = Some(pins);

                    if self.roll_one + pins == 10 {
                        self.status = FrameStatus::Spare;
                    } else {
                        self.status = FrameStatus::Open;
                    }
                }

                Ok(())
            },
            2 => {
                match self.status {
                    FrameStatus::Open => { Err(Error::GameComplete) },
                    FrameStatus::Strike => { 
                        if self.roll_two.unwrap() < 10 && self.roll_two.unwrap() + pins > 10 {
                            return Err(Error::NotEnoughPinsLeft)
                        } else {
                            self.rolls += 1;
                            self.roll_three = Some(pins);
                        }
                        Ok(())
                    },
                    FrameStatus::Spare => { 
                        self.rolls += 1;
                        self.roll_three = Some(pins);
                        Ok(())
                    },
                    FrameStatus::Incomplete => { Err(Error::GameComplete) },
                }
            },
            _ => { Err(Error::GameComplete) }
        }
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame { frames: Vec::new() }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        match self.frames.len() {
            0 => { 
                self.frames.push(Frame::new()); 
                self.frames.last_mut().unwrap().roll(pins)
            },
            9 => {
                match self.frames.last_mut().unwrap().status {
                    FrameStatus::Incomplete => { self.frames.last_mut().unwrap().roll(pins) },
                    _ => { 
                        self.frames.push(Frame::new());
                        self.frames.last_mut().unwrap().roll_tenth_frame(pins)
                    }
                }
            },
            10 => { self.frames.last_mut().unwrap().roll_tenth_frame(pins) },
            _ => { 
                match self.frames.last_mut().unwrap().status {
                    FrameStatus::Incomplete => { self.frames.last_mut().unwrap().roll(pins) },
                    _ => { 
                        self.frames.push(Frame::new());
                        self.frames.last_mut().unwrap().roll(pins)
                    }
                }
            }
        }
    }

    pub fn score(&self) -> Option<u16> {
        match self.frames.len() {
            10 => {
                if self.frames[9].status == FrameStatus::Strike && self.frames[9].roll_three.is_none() {
                    return None
                }
                if self.frames[9].status == FrameStatus::Spare && self.frames[9].roll_three.is_none() {
                    return None
                }
                
                let mut score = 0;
                for i in 0..self.frames.len() {
                    score += score_frame(&self.frames, i).unwrap()
                }

                Some(score)
            }, 
            _ => { None }
        }
    }
}

fn score_frame(frames: &[Frame], idx: usize) -> Option<u16> {
    if idx == 9 {
        if frames[idx].roll_three.is_some() {
            return Some(frames[idx].roll_one + frames[idx].roll_two.unwrap() + frames[idx].roll_three.unwrap())
        } else {
            return Some(frames[idx].roll_one + frames[idx].roll_two.unwrap())
        }
    } else {
        match frames[idx].status {
            FrameStatus::Open => { Some(frames[idx].roll_one + frames[idx].roll_two.unwrap()) },
            FrameStatus::Spare => { Some(10 + frames[idx+1].roll_one) },
            FrameStatus::Strike => { 
                if frames[idx+1].roll_two.is_none() {
                    return Some(10 + frames[idx+1].roll_one + frames[idx+2].roll_one)
                } else {
                    return Some(10 + frames[idx+1].roll_one + frames[idx+2].roll_two.unwrap())
                }
            },
            FrameStatus::Incomplete => { None },
        }
    }
}



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
    pins: u16,
}   

impl Frame {
    fn new() -> Self {
        Frame {
            status: FrameStatus::Incomplete,
            rolls: 0,
            roll_one: 0,
            roll_two: None,
            roll_three: None,
            pins: 0,
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
                self.pins += pins;
                if self.pins == 10 {
                    self.status = FrameStatus::Strike;
                } 

                Ok(())
            },
            1 => { 
                self.rolls += 1;
                self.roll_two = Some(pins);
                self.pins += pins;
                if self.pins > 10 {
                    return Err(Error::NotEnoughPinsLeft)
                }
                if self.pins == 10 {
                    self.status = FrameStatus::Spare;
                    return Ok(())
                } else {
                    self.status = FrameStatus::Open;
                    return Ok(())
                }
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
                self.pins += pins;

                Ok(())
            },
            1 => {
                self.rolls += 1;
                self.roll_two = Some(pins);

                if self.roll_one != 10 && self.roll_one + pins > 10 {
                    return Err(Error::NotEnoughPinsLeft)
                } else if self.roll_one != 10 && self.roll_one + pins < 10 {
                    self.status = FrameStatus::Open;
                    self.pins += pins;
                    return Ok(())
                } else {
                    self.pins += pins;
                    return Ok(())
                }
            },
            2 => {
                // open tenth frame, no bonus roll
                if self.status == FrameStatus::Open {
                    return Ok(())
                }

                // strike and open roll to lead off the tenth frame
                // earns a third roll
                if self.roll_one == 10 && self.roll_two.unwrap() < 10 {
                    self.rolls += 1;
                    self.roll_three = Some(pins);

                    if self.roll_two.unwrap() + self.roll_three.unwrap() > 10 {
                        return Err(Error::NotEnoughPinsLeft)
                    } else {
                        self.pins += pins;
                        return Ok(())
                    }
                }

                // spare to lead off the tenth frame earns a third roll
                if self.roll_one + self.roll_two.unwrap() == 10 {
                    self.status = FrameStatus::Spare;
                    self.rolls += 1;
                    self.roll_three = Some(pins);
                    return Ok(())
                } 
                
                // two strikes to lead off the tenth frame earns a third roll
                if self.roll_one == 10 && self.roll_two.unwrap() == 10 {
                    self.status = FrameStatus::Strike;
                    self.rolls += 1;
                    self.roll_three = Some(pins);
                    return Ok(())
                }

                Ok(())
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
        if self.frames.len() == 0 {
            self.frames.push(Frame::new());
        }

        if self.frames.len() == 10 && self.frames[9].status != FrameStatus::Incomplete {
            return Err(Error::GameComplete)
        } 

        match self.frames.last().unwrap().status {
            FrameStatus::Incomplete => { 
                self.frames.last_mut()
                           .unwrap()
                           .roll(pins) 
            },
            FrameStatus::Strike => { 
                self.frames.push(Frame::new());
                if self.frames.len() == 10 {
                    self.frames.last_mut()
                               .unwrap()
                               .roll_tenth_frame(pins)
                } else {
                    self.frames.last_mut() 
                           .unwrap()
                           .roll(pins)
                }
            },
            FrameStatus::Spare => { 
                self.frames.push(Frame::new());
                if self.frames.len() == 10 {
                    self.frames.last_mut()
                               .unwrap()
                               .roll_tenth_frame(pins)
                } else {
                    self.frames.last_mut() 
                           .unwrap()
                           .roll(pins)
                }
            },
            FrameStatus::Open => { 
                self.frames.push(Frame::new());
                if self.frames.len() == 10 {
                    self.frames.last_mut()
                               .unwrap()
                               .roll_tenth_frame(pins)
                } else {
                    self.frames.last_mut() 
                           .unwrap()
                           .roll(pins)
                }
            },
        }
    }

    pub fn score(&self) -> Option<u16> {
        match self.frames.len() {
            10 => {
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
        return Some(frames[idx].pins)
    } else {
        match frames[idx].status {
            FrameStatus::Open => { Some(frames[idx].pins) },
            FrameStatus::Spare => { Some(10 + frames[idx+1].roll_one) },
            FrameStatus::Strike => { 
                if frames[idx+1].roll_two.is_none() {
                    return Some(10 + frames[idx+1].roll_one + frames[idx+2].roll_one)
                } else {
                    return Some(10 + frames[idx+1].roll_one + frames[idx+1].roll_two.unwrap())
                }
            },
            FrameStatus::Incomplete => { None },
        }
    }
}



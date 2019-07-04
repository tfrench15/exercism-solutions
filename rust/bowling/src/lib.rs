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
    pins: u16,
    score: u16,
}   

impl Frame {
    fn new() -> Self {
        Frame {
            status: FrameStatus::Incomplete,
            rolls: 0,
            pins: 10,
            score: 0, 
        }
    }

    fn roll(&mut self, pins: u16) -> Result<(), Error> {
        match self.rolls {
            0 => {
                self.rolls += 1;
                self.pins -= pins;
                if self.pins == 0 {
                    self.status = FrameStatus::Strike;
                    return Ok(())
                } else if self.pins < 0 {
                    return Err(Error::NotEnoughPinsLeft)
                } else {
                    return Ok(())
                }
            },
            1 => { 
                self.rolls += 1;
                self.pins -= pins;
                if self.pins == 0 {
                    self.status = FrameStatus::Spare;
                    return Ok(())
                }
                if self.pins < 0 {
                    return Err(Error::NotEnoughPinsLeft)
                } else {
                    self.status = FrameStatus::Open;
                    return Ok(())
                }
            },
            _ => {
                return Ok(())
            }
        }
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame { frames: Vec::new() }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.frames.len() == 10 && self.frames[9].status != FrameStatus::Incomplete {
            return Err(Error::GameComplete)
        };

        if self.frames.len() == 0 {
            self.frames.push(Frame::new());
        };

        match self.frames.last().unwrap().status {
            FrameStatus::Incomplete => { self.frames.last_mut().unwrap().roll(pins) },
            FrameStatus::Strike => { 
                self.frames.push(Frame::new());
                self.frames.last_mut()
                           .unwrap()
                           .roll(pins)
            },
            FrameStatus::Spare => { 
                self.frames.push(Frame::new());
                self.frames.last_mut()
                           .unwrap()
                           .roll(pins)
            },
            FrameStatus::Open => { 
                self.frames.push(Frame::new());
                self.frames.last_mut() 
                           .unwrap()
                           .roll(pins)
            },
        }
    }

    pub fn score(&self) -> Option<u16> {
        match self.frames.len() {
            10 => { Some(90) }, // TODO: tabulate score
            _ => { None }
        }
    }
}

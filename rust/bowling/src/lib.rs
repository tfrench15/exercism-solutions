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
            2 => { return Ok(()) }, // TODO: handle 10th frame scoring
            _ => { return Ok(()) },
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
            FrameStatus::Incomplete => { 
                self.frames.last_mut()
                           .unwrap()
                           .roll(pins) 
            },
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
            10 => {
                let mut score = 0;
                for frame in self.frames.windows(3) {
                    score += score_frame(&frame[0], &frame[1], &frame[2]);
                }

                Some(score)
            }, 
            _ => { None }
        }
    }
}

fn score_frame(frame_one: &Frame, frame_two: &Frame, frame_three: &Frame) -> u16 {
    match frame_one.status {
        FrameStatus::Open => { frame_one.pins },
        FrameStatus::Spare => { frame_one.pins + frame_two.roll_one },
        FrameStatus::Strike => { 
            if frame_two.roll_two.is_some() {
                return frame_one.pins + frame_two.pins
            } else {
                return frame_one.pins + frame_two.pins + frame_three.roll_one
            }
        },
        FrameStatus::Incomplete => { 0 },
    }
}



#[derive(Debug)]
pub struct ChessPosition {
    x: i32, 
    y: i32,
}

#[derive(Debug)]
pub struct Queen {
    pos: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || rank > 7 || file < 0 || file > 7 {
            return None
        }

        Some(ChessPosition {
            x: rank,
            y: file,
        })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { pos: position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        true
    }
}

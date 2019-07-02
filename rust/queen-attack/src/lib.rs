#[derive(Debug)]
pub struct ChessPosition {
    rank: i32, 
    file: i32,
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
            rank: rank,
            file: file,
        })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { pos: position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if is_same_file(&self, other) || is_same_rank(&self, other) || is_diagonal(&self, other) {
            return true
        }

        false
    }
}

fn is_same_file(first: &Queen, other: &Queen) -> bool {
    if first.pos.file == other.pos.file {
        return true
    }
    false
}

fn is_same_rank(first: &Queen, other: &Queen) -> bool {
    if first.pos.rank == other.pos.rank {
        return true
    }
    false
}

fn is_diagonal(first: &Queen, other: &Queen) -> bool {
    (first.pos.file - other.pos.file).abs() == (first.pos.rank - other.pos.rank).abs()
}
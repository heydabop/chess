use crate::piece::PieceType;

#[derive(Clone)]
pub struct MoveRecord {
    origin: (u8, u8),
    dest: (u8, u8),
    capture: bool,
    piece_type: PieceType,
}

impl MoveRecord {
    pub fn new(x1: u8, y1: u8, x2: u8, y2: u8, capture: bool, piece_type: PieceType) -> Self {
        Self {
            origin: (x1, y1),
            dest: (x2, y2),
            capture,
            piece_type,
        }
    }

    pub fn origin(&self) -> (u8, u8) {
        self.origin
    }

    pub fn dest(&self) -> (u8, u8) {
        self.dest
    }

    pub fn capture(&self) -> bool {
        self.capture
    }

    pub fn piece_type(&self) -> PieceType {
        self.piece_type
    }
}

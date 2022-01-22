use crate::piece::{Piece, PieceType};

#[derive(Clone, Debug, PartialEq)]
pub struct MoveRecord {
    origin: (u8, u8),
    dest: (u8, u8),
    capture: Option<Piece>,
    piece_type: PieceType,
    first_move: bool, // true if this was the piece's first move
}

impl MoveRecord {
    pub fn new(
        x1: u8,
        y1: u8,
        x2: u8,
        y2: u8,
        capture: Option<Piece>,
        piece_type: PieceType,
        first_move: bool,
    ) -> Self {
        Self {
            origin: (x1, y1),
            dest: (x2, y2),
            capture,
            piece_type,
            first_move,
        }
    }

    pub fn origin(&self) -> (u8, u8) {
        self.origin
    }

    pub fn dest(&self) -> (u8, u8) {
        self.dest
    }

    pub fn is_capture(&self) -> bool {
        self.capture.is_some()
    }

    pub fn take_captured_piece(&mut self) -> Option<Piece> {
        self.capture.take()
    }

    pub fn piece_type(&self) -> PieceType {
        self.piece_type
    }

    pub fn first_move(&self) -> bool {
        self.first_move
    }
}

use crate::color::Color;
use crate::piece::{Piece, PieceType};

pub struct Queen {
    color: Color,
}

impl Queen {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Piece for Queen {
    fn color(&self) -> Color {
        self.color
    }

    fn can_move(&self, x1: u8, y1: u8, x2: u8, y2: u8, piece2: &Option<Box<dyn Piece>>) -> bool {
        true
    }

    fn draw(&self) -> char {
        'Q'
    }

    fn mark_moved(&mut self) {}

    fn piece_type(&self) -> PieceType {
        PieceType::Queen
    }
}

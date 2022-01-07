use crate::color::Color;
use crate::piece::{Piece, PieceType};

pub struct Knight {
    color: Color,
}

impl Knight {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Piece for Knight {
    fn color(&self) -> Color {
        self.color
    }

    fn can_move(&self, x1: u8, y1: u8, x2: u8, y2: u8, piece2: &Option<Box<dyn Piece>>) -> bool {
        true
    }

    fn draw(&self) -> char {
        'N'
    }

    fn mark_moved(&mut self) {}

    fn piece_type(&self) -> PieceType {
        PieceType::Knight
    }
}

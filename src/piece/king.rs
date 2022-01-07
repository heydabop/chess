use crate::color::Color;
use crate::piece::{Piece, PieceType};

pub struct King {
    color: Color,
    has_moved: bool,
}

impl King {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            has_moved: false,
        }
    }
}

impl Piece for King {
    fn color(&self) -> Color {
        self.color
    }

    fn can_move(&self, x1: u8, y1: u8, x2: u8, y2: u8, piece2: &Option<Box<dyn Piece>>) -> bool {
        true
    }

    fn draw(&self) -> char {
        'K'
    }

    fn mark_moved(&mut self) {
        self.has_moved = true;
    }

    fn piece_type(&self) -> PieceType {
        PieceType::King
    }
}
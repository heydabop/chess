use crate::color::Color;
use crate::piece::Piece;

pub struct Pawn {
    color: Color,
    has_moved: bool,
}

impl Pawn {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            has_moved: false,
        }
    }
}

impl Piece for Pawn {
    fn color(&self) -> Color {
        self.color
    }

    fn can_move(&self, x1: u8, y1: u8, x2: u8, y2: u8, piece2: &Option<Box<dyn Piece>>) -> bool {
        true
    }

    fn draw(&self) -> char {
        match self.color {
            Color::Black => '\u{265F}',
            Color::White => '\u{2659}',
        }
    }
}

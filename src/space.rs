use crate::color::Color;
use crate::piece::Piece;

pub struct Space {
    color: Color,
    piece: Option<Box<dyn Piece>>,
}

impl Space {
    pub fn new(color: Color, piece: Option<Box<dyn Piece>>) -> Self {
        Self { color, piece }
    }

    pub fn draw(&self) -> char {
        if let Some(p) = &self.piece {
            p.draw()
        } else {
            '_'
        }
    }

    pub fn piece_color(&self) -> Option<Color> {
        self.piece.as_ref().map(|p| p.color())
    }
}

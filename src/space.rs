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

    pub fn piece(&self) -> &Option<Box<dyn Piece>> {
        &self.piece
    }

    pub fn piece_color(&self) -> Option<Color> {
        self.piece.as_ref().map(|p| p.color())
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn remove_piece(&mut self) -> Option<Box<dyn Piece>> {
        self.piece.take()
    }

    pub fn set_piece(&mut self, piece: Option<Box<dyn Piece>>) {
        self.piece = piece;
    }
}

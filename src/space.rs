use crate::color::Color;
use crate::piece::Piece;

#[derive(Clone, Debug, PartialEq)]
pub struct Space {
    color: Color,
    piece: Option<Piece>,
}

impl Space {
    pub fn new(color: Color, piece: Option<Piece>) -> Self {
        Self { color, piece }
    }

    pub fn draw(&self) -> char {
        if let Some(p) = &self.piece {
            p.draw()
        } else {
            ' '
        }
    }

    pub fn piece(&self) -> &Option<Piece> {
        &self.piece
    }

    pub fn piece_color(&self) -> Option<Color> {
        self.piece.as_ref().map(Piece::color)
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn remove_piece(&mut self) -> Option<Piece> {
        self.piece.take()
    }

    pub fn set_piece(&mut self, piece: Option<Piece>) {
        self.piece = piece;
    }
}

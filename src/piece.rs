use crate::color::Color;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Piece {
    color: Color,
    has_moved: bool,
    r#type: PieceType,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Self {
            color,
            r#type: piece_type,
            has_moved: false,
        }
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn draw(&self) -> char {
        match self.r#type {
            PieceType::Pawn => 'P',
            PieceType::Knight => 'N',
            PieceType::Queen => 'Q',
            PieceType::King => 'K',
            PieceType::Bishop => 'B',
            PieceType::Rook => 'R',
        }
    }

    pub fn mark_moved(&mut self) {
        self.has_moved = true;
    }

    pub fn unmark_moved(&mut self) {
        self.has_moved = false;
    }

    pub fn has_moved(&self) -> bool {
        self.has_moved
    }

    pub fn piece_type(&self) -> PieceType {
        self.r#type
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

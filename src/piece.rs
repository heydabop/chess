use crate::color::Color;

#[derive(Clone, Debug, PartialEq)]
pub struct Piece {
    color: Color,
    has_moved: bool,
    piece_type: PieceType,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Self {
            color,
            piece_type,
            has_moved: false,
        }
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn draw(&self) -> char {
        match self.piece_type {
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

    pub fn has_moved(&self) -> bool {
        self.has_moved
    }

    pub fn piece_type(&self) -> PieceType {
        self.piece_type
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

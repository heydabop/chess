use crate::color::Color;

#[derive(Clone, Debug, PartialEq, Eq)]
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

    pub fn unmark_moved(&mut self) {
        self.has_moved = false;
    }

    pub fn has_moved(&self) -> bool {
        self.has_moved
    }

    pub fn piece_type(&self) -> PieceType {
        self.piece_type
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PieceType {
    King = 1,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl From<u8> for PieceType {
    fn from(i: u8) -> Self {
        match i {
            1 => Self::King,
            2 => Self::Queen,
            3 => Self::Rook,
            4 => Self::Bishop,
            5 => Self::Knight,
            6 => Self::Pawn,
            _ => panic!("unrecognized piece type {i}"),
        }
    }
}

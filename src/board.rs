use crate::color::Color;
use crate::pawn::Pawn;
use crate::piece::Piece;
use crate::space::Space;
use std::array::from_fn;

pub struct Board {
    spaces: [[Space; 8]; 8],
}

impl Board {
    pub fn new() -> Self {
        Self {
            spaces: from_fn(|row| {
                from_fn(|col| {
                    let color = if (row + col) % 2 == 0 {
                        Color::Black
                    } else {
                        Color::White
                    };
                    let piece: Option<Box<dyn Piece>> = if row == 0 {
                        Some(Box::new(Pawn::new(Color::White)))
                    } else if row == 7 {
                        Some(Box::new(Pawn::new(Color::Black)))
                    } else {
                        None
                    };
                    Space::new(color, piece)
                })
            }),
        }
    }

    pub fn spaces(&self) -> &[[Space; 8]; 8] {
        &self.spaces
    }
}

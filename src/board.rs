use crate::color::Color;
use crate::pawn::Pawn;
use crate::piece::Piece;
use crate::space::Space;
use std::array::from_fn;

pub struct Board {
    spaces: [[Space; 8]; 8],
    turn_color: Color,
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
            turn_color: Color::White,
        }
    }

    pub fn space(&self, x: u8, y: u8) -> &Space {
        &self.spaces[y as usize][x as usize]
    }

    pub fn spaces(&self) -> &[[Space; 8]; 8] {
        &self.spaces
    }

    pub fn turn_color(&self) -> Color {
        self.turn_color
    }

    pub fn next_turn(&mut self) {
        self.turn_color = match self.turn_color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
    }

    pub fn move_piece(&mut self, x1: u8, y1: u8, x2: u8, y2: u8) -> bool {
        let piece = self.space(x1, y1).piece();
        if let Some(piece) = &piece {
            let piece2 = self.space(x2, y2).piece();
            if !piece.can_move(x1, y1, x2, y2, piece2) {
                return false;
            }
        } else {
            return false;
        }
        let mut piece = self.spaces[y1 as usize][x1 as usize]
            .remove_piece()
            .unwrap();
        piece.mark_moved();
        self.spaces[y2 as usize][x2 as usize].set_piece(Some(piece));
        true
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

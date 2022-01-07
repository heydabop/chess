use crate::color::Color;
use crate::move_record::MoveRecord;
use crate::pawn::Pawn;
use crate::piece::{Piece, PieceType};
use crate::space::Space;
use std::array::from_fn;

pub struct Board {
    spaces: [[Space; 8]; 8],
    turn_color: Color,
    moves: Vec<MoveRecord>,
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
                    let piece: Option<Box<dyn Piece>> = if row == 1 {
                        Some(Box::new(Pawn::new(Color::White)))
                    } else if row == 6 {
                        Some(Box::new(Pawn::new(Color::Black)))
                    } else {
                        None
                    };
                    Space::new(color, piece)
                })
            }),
            turn_color: Color::White,
            moves: vec![],
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
        let piece2 = self.space(x2, y2).piece();
        if let Some(piece) = &piece {
            // Check for en passant here since it requires more board state than piece.can_move
            // The current piece is a pawn, there is no piece in its destination, it's on its fifth file and moving forward into its sixth
            if piece.piece_type() == PieceType::Pawn
                && piece2.is_none()
                && ((self.turn_color == Color::White && y1 == 4 && y2 == 5)
                    || (self.turn_color == Color::Black && y1 == 3 && y2 == 2))
            {
                if let Some(last_move) = self.moves.last() {
                    // there was a previous move
                    if last_move.piece_type() == PieceType::Pawn // last move was a pawn
                        && (x1 + 1 == x2 || x2 + 1 == x1) // moving pawn is moving diagonally (we already checked it's moving forward, checking left/right here)
                        && last_move.origin().0 == last_move.dest().0 // the last move was directly forward
                        && (last_move.origin().1 + 2 == last_move.dest().1 || last_move.dest().1 + 2 == last_move.origin().1) // the last move was across two ranks
                        && x2 == last_move.dest().0
                    // the current move is moving into the last moves file
                    {
                        let last_dest = last_move.dest();
                        let mut piece = self.spaces[y1 as usize][x1 as usize]
                            .remove_piece()
                            .unwrap();
                        piece.mark_moved();
                        self.moves
                            .push(MoveRecord::new(x1, y1, x2, y2, true, piece.piece_type()));
                        self.spaces[y2 as usize][x2 as usize].set_piece(Some(piece));
                        self.spaces[last_dest.1 as usize][last_dest.0 as usize].set_piece(None); // remove last_move pawn
                        return true;
                    }
                }
            }
            if !piece.can_move(x1, y1, x2, y2, piece2) {
                return false;
            }
        } else {
            return false;
        }
        let capture = piece2.is_some();
        let mut piece = self.spaces[y1 as usize][x1 as usize]
            .remove_piece()
            .unwrap();
        piece.mark_moved();
        self.moves
            .push(MoveRecord::new(x1, y1, x2, y2, capture, piece.piece_type()));
        self.spaces[y2 as usize][x2 as usize].set_piece(Some(piece));
        true
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

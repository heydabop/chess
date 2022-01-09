use crate::color::Color;
use crate::move_record::MoveRecord;
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
                    let piece: Option<Piece> = match row {
                        0 => match col {
                            0 | 7 => Some(Piece::new(PieceType::Rook, Color::White)),
                            1 | 6 => Some(Piece::new(PieceType::Knight, Color::White)),
                            2 | 5 => Some(Piece::new(PieceType::Bishop, Color::White)),
                            3 => Some(Piece::new(PieceType::Queen, Color::White)),
                            4 => Some(Piece::new(PieceType::King, Color::White)),
                            _ => panic!("Board generation column out of bounds"),
                        },
                        1 => Some(Piece::new(PieceType::Pawn, Color::White)),
                        2..=5 => None,
                        6 => Some(Piece::new(PieceType::Pawn, Color::Black)),
                        7 => match col {
                            0 | 7 => Some(Piece::new(PieceType::Rook, Color::Black)),
                            1 | 6 => Some(Piece::new(PieceType::Knight, Color::Black)),
                            2 | 5 => Some(Piece::new(PieceType::Bishop, Color::Black)),
                            3 => Some(Piece::new(PieceType::Queen, Color::Black)),
                            4 => Some(Piece::new(PieceType::King, Color::Black)),
                            _ => panic!("Board generation column out of bounds"),
                        },
                        _ => panic!("Board generation row out of bounds"),
                    };
                    Space::new(color, piece)
                })
            }),
            turn_color: Color::White,
            moves: vec![],
        }
    }

    pub fn make_custom(placements: Vec<(Piece, u8, u8)>, starting_color: Color) -> Self {
        let mut spaces = from_fn(|row| {
            from_fn(|col| {
                let color = if (row + col) % 2 == 0 {
                    Color::Black
                } else {
                    Color::White
                };
                Space::new(color, None)
            })
        });
        for p in placements {
            let space_color = spaces[p.2 as usize][p.1 as usize].color();
            spaces[p.2 as usize][p.1 as usize] = Space::new(space_color, Some(p.0));
        }
        Self {
            spaces,
            turn_color: starting_color,
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

        // Check and execute en passant here since piece removal from capture is different than normal
        if piece.as_ref().map(Piece::piece_type) == Some(PieceType::Pawn)
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
                // the current move is moving into the last pawn's file
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

        if !match piece.as_ref().map(Piece::piece_type) {
            None => false,
            Some(PieceType::Pawn) => self.pawn_can_move(x1, y1, x2, y2),
            _ => false,
        } {
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

    fn pawn_can_move(&self, x1: u8, y1: u8, x2: u8, y2: u8) -> bool {
        let piece = self.space(x1, y1).piece().as_ref().unwrap();
        let piece2 = self.space(x2, y2).piece();
        match piece.color() {
            Color::White => {
                (!piece.has_moved() && y1 + 2 == y2 && x1 == x2 && piece2.is_none())
                    || (y1 + 1 == y2
                        && ((x1 == x2 && piece2.is_none())
                            || ((x1 + 1 == x2 || x2 + 1 == x1)
                                && piece2.is_some()
                                && piece2.as_ref().unwrap().color() == Color::Black)))
            }
            Color::Black => {
                (!piece.has_moved() && y2 + 2 == y1 && x1 == x2 && piece2.is_none())
                    || (y2 + 1 == y1
                        && ((x1 == x2 && piece2.is_none())
                            || ((x1 + 1 == x2 || x2 + 1 == x1)
                                && piece2.is_some()
                                && piece2.as_ref().unwrap().color() == Color::White)))
            }
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pawn_can_move_forward_into_empty() {
        let wp = Piece::new(PieceType::Pawn, Color::White);
        let bp = Piece::new(PieceType::Pawn, Color::Black);
        let b = Board::make_custom(vec![(wp, 0, 1)], Color::White);
        assert_eq!(b.pawn_can_move(0, 1, 0, 2), true);
        let b = Board::make_custom(vec![(bp, 0, 6)], Color::Black);
        assert_eq!(b.pawn_can_move(0, 6, 0, 5), true);
    }

    #[test]
    fn pawn_can_attack() {
        let wp = Piece::new(PieceType::Pawn, Color::White);
        let bp = Piece::new(PieceType::Pawn, Color::Black);
        let b = Board::make_custom(vec![(wp.clone(), 0, 1), (bp.clone(), 1, 2)], Color::White);
        assert_eq!(b.pawn_can_move(0, 1, 1, 2), true);
        let b = Board::make_custom(vec![(wp, 0, 5), (bp, 1, 6)], Color::Black);
        assert_eq!(b.pawn_can_move(1, 6, 0, 5), true);
    }

    #[test]
    fn pawn_cannot_move_forward_into_occupied() {
        let wp = Piece::new(PieceType::Pawn, Color::White);
        let bp = Piece::new(PieceType::Pawn, Color::Black);
        let b = Board::make_custom(vec![(wp.clone(), 0, 1), (bp.clone(), 0, 2)], Color::White);
        assert_eq!(b.pawn_can_move(0, 1, 0, 2), false);
        let b = Board::make_custom(vec![(wp.clone(), 0, 5), (bp.clone(), 0, 6)], Color::Black);
        assert_eq!(b.pawn_can_move(0, 6, 0, 5), false);
        let b = Board::make_custom(vec![(wp.clone(), 0, 1), (bp.clone(), 0, 3)], Color::White);
        assert_eq!(b.pawn_can_move(0, 1, 0, 3), false);
        let b = Board::make_custom(vec![(wp, 0, 4), (bp, 0, 6)], Color::Black);
        assert_eq!(b.pawn_can_move(0, 6, 0, 4), false);
    }

    #[test]
    fn pawn_can_move_forward_two() {
        let wp = Piece::new(PieceType::Pawn, Color::White);
        let bp = Piece::new(PieceType::Pawn, Color::Black);
        let b = Board::make_custom(vec![(wp, 0, 1)], Color::White);
        assert_eq!(b.pawn_can_move(0, 1, 0, 3), true);
        let b = Board::make_custom(vec![(bp, 0, 6)], Color::Black);
        assert_eq!(b.pawn_can_move(0, 6, 0, 4), true);
    }

    #[test]
    fn pawn_cannot_move_forward_two_after_move() {
        let mut wp = Piece::new(PieceType::Pawn, Color::White);
        let mut bp = Piece::new(PieceType::Pawn, Color::Black);
        wp.mark_moved();
        bp.mark_moved();
        let b = Board::make_custom(vec![(wp, 0, 1)], Color::White);
        assert_eq!(b.pawn_can_move(0, 1, 0, 3), false);
        let b = Board::make_custom(vec![(bp, 0, 6)], Color::Black);
        assert_eq!(b.pawn_can_move(0, 6, 0, 4), false);
    }

    #[test]
    fn pawn_cannot_attack_into_empty() {
        let wp = Piece::new(PieceType::Pawn, Color::White);
        let bp = Piece::new(PieceType::Pawn, Color::Black);
        let b = Board::make_custom(vec![(wp, 0, 1)], Color::White);
        assert_eq!(b.pawn_can_move(0, 1, 1, 2), false);
        let b = Board::make_custom(vec![(bp, 1, 6)], Color::Black);
        assert_eq!(b.pawn_can_move(1, 6, 0, 5), false);
    }

    #[test]
    fn pawn_cannot_move_two_and_attack() {
        let wp = Piece::new(PieceType::Pawn, Color::White);
        let bp = Piece::new(PieceType::Pawn, Color::Black);
        let b = Board::make_custom(vec![(wp.clone(), 0, 1), (bp.clone(), 1, 3)], Color::White);
        assert_eq!(b.pawn_can_move(0, 1, 1, 3), false);
        let b = Board::make_custom(vec![(wp.clone(), 0, 4), (bp.clone(), 1, 6)], Color::Black);
        assert_eq!(b.pawn_can_move(1, 6, 0, 4), false);
        let b = Board::make_custom(vec![(wp, 0, 1)], Color::White);
        assert_eq!(b.pawn_can_move(0, 1, 1, 3), false);
        let b = Board::make_custom(vec![(bp, 1, 6)], Color::Black);
        assert_eq!(b.pawn_can_move(1, 6, 0, 4), false);
    }

    #[test]
    fn pawn_cannot_move_backwards() {
        let wp = Piece::new(PieceType::Pawn, Color::White);
        let bp = Piece::new(PieceType::Pawn, Color::Black);
        let b = Board::make_custom(vec![(wp, 0, 1)], Color::White);
        assert_eq!(b.pawn_can_move(0, 1, 0, 0), false);
        let b = Board::make_custom(vec![(bp, 0, 6)], Color::Black);
        assert_eq!(b.pawn_can_move(0, 6, 0, 7), false);
    }
}

use crate::color::Color;
use crate::move_record::MoveRecord;
use crate::piece::{Piece, PieceType};
use crate::space::Space;
use std::array::from_fn;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    spaces: [[Space; 8]; 8],
    turn_color: Color,
    moves: Vec<MoveRecord>,
    captured_by_white: HashMap<PieceType, u8>,
    captured_by_black: HashMap<PieceType, u8>,
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
            captured_by_white: HashMap::new(),
            captured_by_black: HashMap::new(),
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
            captured_by_white: HashMap::new(),
            captured_by_black: HashMap::new(),
        }
    }

    // TODO: this only sets piece placements and turn color, it doesnt set moves or mark pieces as having moved
    pub fn from_strs(state: &[&str]) -> Self {
        assert!(
            state.len() == 9,
            "Expected 9 strs in board state, got {}",
            state.len()
        );
        let spaces = from_fn(|row| {
            let chars: Vec<char> = state[7 - row].chars().collect();
            assert!(
                chars.len() == 8,
                "Expected each row to be 8 chars, found row with {} chars",
                chars.len()
            );
            from_fn(|col| {
                let color = if (row + col) % 2 == 0 {
                    Color::Black
                } else {
                    Color::White
                };
                #[allow(clippy::match_on_vec_items)]
                let piece = match chars[col] {
                    '_' => None,
                    'K' => Some(Piece::new(PieceType::King, Color::White)),
                    'k' => Some(Piece::new(PieceType::King, Color::Black)),
                    'Q' => Some(Piece::new(PieceType::Queen, Color::White)),
                    'q' => Some(Piece::new(PieceType::Queen, Color::Black)),
                    'R' => Some(Piece::new(PieceType::Rook, Color::White)),
                    'r' => Some(Piece::new(PieceType::Rook, Color::Black)),
                    'B' => Some(Piece::new(PieceType::Bishop, Color::White)),
                    'b' => Some(Piece::new(PieceType::Bishop, Color::Black)),
                    'N' => Some(Piece::new(PieceType::Knight, Color::White)),
                    'n' => Some(Piece::new(PieceType::Knight, Color::Black)),
                    'P' => Some(Piece::new(PieceType::Pawn, Color::White)),
                    'p' => Some(Piece::new(PieceType::Pawn, Color::Black)),
                    _ => panic!("Unrecognized character in board state str"),
                };
                Space::new(color, piece)
            })
        });
        let turn_color = match state[8].chars().next() {
            Some('W') => Color::White,
            Some('B') => Color::Black,
            _ => panic!("Unrecognized character in board state color"),
        };
        Self {
            spaces,
            turn_color,
            moves: vec![],
            captured_by_white: HashMap::new(),
            captured_by_black: HashMap::new(),
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

    fn toggle_turn(&mut self) {
        self.turn_color = match self.turn_color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
    }

    pub fn move_piece(&mut self, x1: u8, y1: u8, x2: u8, y2: u8) -> bool {
        if x1 == x2 && y1 == y2 {
            return false;
        }
        let piece = self.space(x1, y1).piece();
        if piece.is_none() {
            return false;
        }
        let piece = piece.as_ref().unwrap();
        let color = piece.color();
        if color != self.turn_color {
            return false;
        }
        let piece2 = self.space(x2, y2).piece();

        // Check and execute en passant here since piece removal from capture is different than normal
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
                // the current move is moving into the last pawn's file
                {
                    let last_dest = last_move.dest();
                    let mut piece = self.spaces[y1 as usize][x1 as usize]
                        .remove_piece()
                        .unwrap();
                    let piece2 = self.spaces[last_dest.1 as usize][last_dest.0 as usize]
                        .remove_piece()
                        .unwrap(); //get and remove last_move pawn
                    self.record_capture_by(piece.color(), piece2.piece_type());
                    self.moves.push(MoveRecord::new(
                        x1,
                        y1,
                        x2,
                        y2,
                        Some(piece2),
                        piece.piece_type(),
                        !piece.has_moved(),
                    ));
                    piece.mark_moved();
                    self.spaces[y2 as usize][x2 as usize].set_piece(Some(piece));
                    self.toggle_turn();

                    // undo this move if it has put the player in check
                    if self.is_in_check(color) {
                        self.undo_last_move();
                        return false;
                    }
                    return true;
                }
            }
        }

        // Check and execute castling here since piece movement is different than normal
        if piece.piece_type() == PieceType::King
            && !piece.has_moved()
            && (x1 + 2 == x2 || x2 + 2 == x1)
            && piece2.is_none()
        {
            let rank = match color {
                Color::White => 0,
                Color::Black => 7,
            };
            if y1 == rank && y2 == rank {
                let rook = if x1 + 2 == x2 {
                    self.space(7, rank).piece()
                } else {
                    //we already checked that either x1 + 2 == x2 or x2 + 2 == x1
                    self.space(0, rank).piece()
                };
                if let Some(rook) = rook {
                    if !rook.has_moved() && rook.color() == color {
                        // king cannot move out of, through, or into check
                        if self.is_space_attacked(x1, y1, color) {
                            return false;
                        }
                        if x1 + 2 == x2 {
                            if self.is_space_attacked(x1 + 1, y1, color)
                                || self.is_space_attacked(x2, y1, color)
                                || self.space(x1 + 1, y1).piece().is_some()
                            {
                                return false;
                            }
                        } else if self.is_space_attacked(x1 - 1, y1, color)
                            || self.is_space_attacked(x2, y1, color)
                            || self.space(x1 - 1, y1).piece().is_some()
                        {
                            return false;
                        }
                        let mut piece = self.spaces[y1 as usize][x1 as usize]
                            .remove_piece()
                            .unwrap();
                        self.moves.push(MoveRecord::new(
                            x1,
                            y1,
                            x2,
                            y2,
                            None,
                            piece.piece_type(),
                            !piece.has_moved(),
                        ));
                        piece.mark_moved();
                        self.spaces[y2 as usize][x2 as usize].set_piece(Some(piece));
                        if x1 + 2 == x2 {
                            let mut rook = self.spaces[y1 as usize][7].remove_piece().unwrap();
                            rook.mark_moved();
                            self.spaces[y1 as usize][5].set_piece(Some(rook));
                        } else {
                            let mut rook = self.spaces[y1 as usize][0].remove_piece().unwrap();
                            rook.mark_moved();
                            self.spaces[y1 as usize][3].set_piece(Some(rook));
                        }
                        self.toggle_turn();
                        // undo this move if it has put the player in check (tho castling should check for this already)
                        if self.is_in_check(color) {
                            self.undo_last_move();
                            return false;
                        }
                        return true;
                    }
                }
            }
        }

        if !match piece.piece_type() {
            PieceType::Pawn => self.pawn_can_move(x1, y1, x2, y2),
            PieceType::Rook => self.rook_can_move(x1, y1, x2, y2),
            PieceType::Bishop => self.bishop_can_move(x1, y1, x2, y2),
            PieceType::Queen => self.queen_can_move(x1, y1, x2, y2),
            PieceType::King => self.king_can_move(x1, y1, x2, y2),
            PieceType::Knight => self.knight_can_move(x1, y1, x2, y2),
        } {
            return false;
        }

        let mut piece = self.spaces[y1 as usize][x1 as usize]
            .remove_piece()
            .unwrap();
        let piece2 = self.spaces[y2 as usize][x2 as usize].remove_piece();
        if let Some(piece2) = &piece2 {
            self.record_capture_by(piece.color(), piece2.piece_type());
        }
        self.moves.push(MoveRecord::new(
            x1,
            y1,
            x2,
            y2,
            piece2,
            piece.piece_type(),
            !piece.has_moved(),
        ));
        piece.mark_moved();
        self.spaces[y2 as usize][x2 as usize].set_piece(Some(piece));
        self.toggle_turn();

        // undo this move if it has put the player in check
        if self.is_in_check(color) {
            self.undo_last_move();
            return false;
        }

        true
    }

    pub fn promote_pawn(&mut self, x: u8, y: u8, piece_type: PieceType) {
        let piece = self.spaces[y as usize][x as usize]
            .remove_piece()
            .expect("promote called on space without piece");
        assert_eq!(
            piece.piece_type(),
            PieceType::Pawn,
            "promote called on non-pawn"
        );
        assert!(
            (piece.color() == Color::White && y == 7) || (piece.color() == Color::Black && y == 0),
            "promote called on ineligible pawn"
        );
        self.spaces[y as usize][x as usize].set_piece(Some(Piece::new(piece_type, piece.color())));
    }

    pub fn undo_last_move(&mut self) {
        if self.moves.is_empty() {
            return;
        }
        let mut last_move = self.moves.pop().unwrap();
        let (x1, y1) = last_move.origin();
        let (x2, y2) = last_move.dest();
        let mut piece = self.spaces[y2 as usize][x2 as usize]
            .remove_piece()
            .unwrap();
        let is_castle = piece.piece_type() == PieceType::King
            && y1 == y2
            && (i16::from(x1) - i16::from(x2)).abs() == 2;
        if last_move.first_move() {
            piece.unmark_moved();
        }
        if last_move.is_capture() {
            let piece2 = last_move
                .take_captured_piece()
                .expect("expected target piece when undoing last capture");
            if let Some(captures) = match piece.color() {
                Color::White => self.captured_by_white.get_mut(&piece2.piece_type()),
                Color::Black => self.captured_by_black.get_mut(&piece2.piece_type()),
            } {
                *captures -= 1;
                if *captures == 0 {
                    match piece.color() {
                        Color::White => self.captured_by_white.remove(&piece2.piece_type()),
                        Color::Black => self.captured_by_black.remove(&piece2.piece_type()),
                    };
                }
            }
            self.spaces[y2 as usize][x2 as usize].set_piece(Some(piece2));
        } else if is_castle {
            // move rook as well
            let (rook_x1, rook_x2) = if x1 < x2 {
                (7usize, x2 - 1)
            } else {
                (0usize, x2 + 1)
            };
            let mut rook = self.spaces[y2 as usize][rook_x2 as usize]
                .remove_piece()
                .unwrap();
            rook.unmark_moved(); // can only castle if rook was unmoved, reset this
            self.spaces[y1 as usize][rook_x1].set_piece(Some(rook));
        }
        self.spaces[y1 as usize][x1 as usize].set_piece(Some(piece));
        self.toggle_turn();
    }

    fn record_capture_by(&mut self, color: Color, captured_piece_type: PieceType) {
        let count = match color {
            Color::White => self
                .captured_by_white
                .entry(captured_piece_type)
                .or_insert(0),
            Color::Black => self
                .captured_by_black
                .entry(captured_piece_type)
                .or_insert(0),
        };
        *count += 1;
    }

    fn pawn_can_move(&self, x1: u8, y1: u8, x2: u8, y2: u8) -> bool {
        let piece = self.space(x1, y1).piece().as_ref().unwrap();
        assert!(
            !(piece.piece_type() != PieceType::Pawn),
            "pawn_can_move called on {:?}",
            piece.piece_type()
        );
        let piece2 = self.space(x2, y2).piece();
        match piece.color() {
            Color::White => {
                (!piece.has_moved()
                    && y1 + 2 == y2
                    && x1 == x2
                    && piece2.is_none()
                    && self.space(x1, y2 - 1).piece().is_none())
                    || (y1 + 1 == y2
                        && ((x1 == x2 && piece2.is_none())
                            || ((x1 + 1 == x2 || x2 + 1 == x1)
                                && piece2.is_some()
                                && piece2.as_ref().unwrap().color() == Color::Black)))
            }
            Color::Black => {
                (!piece.has_moved()
                    && y2 + 2 == y1
                    && x1 == x2
                    && piece2.is_none()
                    && self.space(x1, y1 - 1).piece().is_none())
                    || (y2 + 1 == y1
                        && ((x1 == x2 && piece2.is_none())
                            || ((x1 + 1 == x2 || x2 + 1 == x1)
                                && piece2.is_some()
                                && piece2.as_ref().unwrap().color() == Color::White)))
            }
        }
    }

    fn rook_can_move(&self, x1: u8, y1: u8, x2: u8, y2: u8) -> bool {
        let piece = self.space(x1, y1).piece().as_ref().unwrap();
        assert!(
            !(piece.piece_type() != PieceType::Rook && piece.piece_type() != PieceType::Queen),
            "rook_can_move called on {:?}",
            piece.piece_type()
        );
        // If there is a piece at the destination and its the same color
        if self.space(x2, y2).piece().as_ref().map(Piece::color) == Some(piece.color()) {
            return false;
        }
        // If the move isn't along a single rank or file
        if x1 != x2 && y1 != y2 {
            return false;
        }
        // Check that there aren't pieces between the origin and destination
        if x1 == x2 {
            if y1 < y2 {
                for i in y1 + 1..y2 {
                    if self.space(x1, i).piece().is_some() {
                        return false;
                    }
                }
            } else {
                for i in y2 + 1..y1 {
                    if self.space(x1, i).piece().is_some() {
                        return false;
                    }
                }
            }
        } else {
            // y1 == y2
            if x1 < x2 {
                for i in x1 + 1..x2 {
                    if self.space(i, y1).piece().is_some() {
                        return false;
                    }
                }
            } else {
                for i in x2 + 1..x1 {
                    if self.space(i, y1).piece().is_some() {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn bishop_can_move(&self, x1: u8, y1: u8, x2: u8, y2: u8) -> bool {
        let piece = self.space(x1, y1).piece().as_ref().unwrap();
        assert!(
            !(piece.piece_type() != PieceType::Bishop && piece.piece_type() != PieceType::Queen),
            "bishop_can_move called on {:?}",
            piece.piece_type()
        );
        // If there is a piece at the destination and its the same color
        if self.space(x2, y2).piece().as_ref().map(Piece::color) == Some(piece.color()) {
            return false;
        }
        // If the move isn't diagonal
        if (i16::from(x1) - i16::from(x2)).abs() != (i16::from(y1) - i16::from(y2)).abs() {
            return false;
        }
        // Check that there aren't pieces between the origin and destination
        if x1 < x2 {
            if y1 < y2 {
                for i in 1..x2 - x1 {
                    if self.space(x1 + i, y1 + i).piece().is_some() {
                        return false;
                    }
                }
            } else {
                for i in 1..x2 - x1 {
                    if self.space(x1 + i, y1 - i).piece().is_some() {
                        return false;
                    }
                }
            }
        } else if y1 < y2 {
            for i in 1..x1 - x2 {
                if self.space(x1 - i, y1 + i).piece().is_some() {
                    return false;
                }
            }
        } else {
            for i in 1..x1 - x2 {
                if self.space(x1 - i, y1 - i).piece().is_some() {
                    return false;
                }
            }
        }
        true
    }

    fn queen_can_move(&self, x1: u8, y1: u8, x2: u8, y2: u8) -> bool {
        self.rook_can_move(x1, y1, x2, y2) || self.bishop_can_move(x1, y1, x2, y2)
    }

    fn king_can_move(&self, x1: u8, y1: u8, x2: u8, y2: u8) -> bool {
        let piece = self.space(x1, y1).piece().as_ref().unwrap();
        assert!(
            !(piece.piece_type() != PieceType::King),
            "king_can_move called on {:?}",
            piece.piece_type()
        );
        // If there is a piece at the destination and its the same color
        if self.space(x2, y2).piece().as_ref().map(Piece::color) == Some(piece.color()) {
            return false;
        }
        let x_abs = (i16::from(x1) - i16::from(x2)).abs();
        let y_abs = (i16::from(y1) - i16::from(y2)).abs();
        if x_abs > 1 || y_abs > 1 {
            return false;
        }
        if self.is_space_attacked(x2, y2, piece.color()) {
            return false;
        }
        true
    }

    fn knight_can_move(&self, x1: u8, y1: u8, x2: u8, y2: u8) -> bool {
        let piece = self.space(x1, y1).piece().as_ref().unwrap();
        assert!(
            !(piece.piece_type() != PieceType::Knight),
            "knight_can_move called on {:?}",
            piece.piece_type()
        );
        // If there is a piece at the destination and its the same color
        if self.space(x2, y2).piece().as_ref().map(Piece::color) == Some(piece.color()) {
            return false;
        }
        let x_abs = (i16::from(x1) - i16::from(x2)).abs();
        let y_abs = (i16::from(y1) - i16::from(y2)).abs();
        (x_abs == 2 && y_abs == 1) || (x_abs == 1 && y_abs == 2)
    }

    fn is_space_attacked(&self, x: u8, y: u8, color: Color) -> bool {
        for x0 in 0..8 {
            for y0 in 0..8 {
                if x0 == x && y0 == y {
                    continue;
                }
                if let Some(piece) = self.space(x0, y0).piece() {
                    if piece.color() != color
                        && match piece.piece_type() {
                            PieceType::Pawn => self.pawn_can_move(x0, y0, x, y),
                            PieceType::Rook => self.rook_can_move(x0, y0, x, y),
                            PieceType::Bishop => self.bishop_can_move(x0, y0, x, y),
                            PieceType::Queen => self.queen_can_move(x0, y0, x, y),
                            PieceType::King => self.king_can_move(x0, y0, x, y),
                            PieceType::Knight => self.knight_can_move(x0, y0, x0, y),
                        }
                    {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn is_in_check(&self, color: Color) -> bool {
        //find king
        let pos = self.spaces.iter().enumerate().find_map(|(y, row)| {
            return row.iter().enumerate().find_map(|(x, space)| {
                if let Some(piece) = space.piece() {
                    if piece.piece_type() == PieceType::King && piece.color() == color {
                        #[allow(clippy::cast_possible_truncation)]
                        return Some((x as u8, y as u8));
                    }
                }
                None
            });
        });
        // should this panic or simply return false? need to not panic if there are custom boards
        let pos = pos.unwrap_or_else(|| panic!("unable to find {:?} king on board", color));

        self.is_space_attacked(pos.0, pos.1, color)
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
        assert!(b.pawn_can_move(0, 1, 0, 2));
        let b = Board::make_custom(vec![(bp, 0, 6)], Color::Black);
        assert!(b.pawn_can_move(0, 6, 0, 5));
    }

    #[test]
    fn pawn_can_capture() {
        let wp = Piece::new(PieceType::Pawn, Color::White);
        let bp = Piece::new(PieceType::Pawn, Color::Black);
        let b = Board::make_custom(vec![(wp.clone(), 0, 1), (bp.clone(), 1, 2)], Color::White);
        assert!(b.pawn_can_move(0, 1, 1, 2));
        let b = Board::make_custom(vec![(wp, 0, 5), (bp, 1, 6)], Color::Black);
        assert!(b.pawn_can_move(1, 6, 0, 5));
    }

    #[test]
    fn pawn_cannot_move_forward_into_occupied() {
        let wp = Piece::new(PieceType::Pawn, Color::White);
        let bp = Piece::new(PieceType::Pawn, Color::Black);
        let b = Board::make_custom(vec![(wp.clone(), 0, 1), (bp.clone(), 0, 2)], Color::White);
        assert!(!b.pawn_can_move(0, 1, 0, 2));
        let b = Board::make_custom(vec![(wp.clone(), 0, 5), (bp.clone(), 0, 6)], Color::Black);
        assert!(!b.pawn_can_move(0, 6, 0, 5));
        let b = Board::make_custom(vec![(wp.clone(), 0, 1), (bp.clone(), 0, 3)], Color::White);
        assert!(!b.pawn_can_move(0, 1, 0, 3));
        let b = Board::make_custom(vec![(wp, 0, 4), (bp, 0, 6)], Color::Black);
        assert!(!b.pawn_can_move(0, 6, 0, 4));
    }

    #[test]
    fn pawn_can_move_forward_two() {
        let wp = Piece::new(PieceType::Pawn, Color::White);
        let bp = Piece::new(PieceType::Pawn, Color::Black);
        let b = Board::make_custom(vec![(wp, 0, 1)], Color::White);
        assert!(b.pawn_can_move(0, 1, 0, 3));
        let b = Board::make_custom(vec![(bp, 0, 6)], Color::Black);
        assert!(b.pawn_can_move(0, 6, 0, 4));
    }

    #[test]
    fn pawn_cannot_move_forward_two_after_move() {
        let mut wp = Piece::new(PieceType::Pawn, Color::White);
        let mut bp = Piece::new(PieceType::Pawn, Color::Black);
        wp.mark_moved();
        bp.mark_moved();
        let b = Board::make_custom(vec![(wp, 0, 1)], Color::White);
        assert!(!b.pawn_can_move(0, 1, 0, 3));
        let b = Board::make_custom(vec![(bp, 0, 6)], Color::Black);
        assert!(!b.pawn_can_move(0, 6, 0, 4));
    }

    #[test]
    fn pawn_cannot_capture_into_empty() {
        let wp = Piece::new(PieceType::Pawn, Color::White);
        let bp = Piece::new(PieceType::Pawn, Color::Black);
        let b = Board::make_custom(vec![(wp, 0, 1)], Color::White);
        assert!(!b.pawn_can_move(0, 1, 1, 2));
        let b = Board::make_custom(vec![(bp, 1, 6)], Color::Black);
        assert!(!b.pawn_can_move(1, 6, 0, 5));
    }

    #[test]
    fn pawn_cannot_move_two_and_capture() {
        let wp = Piece::new(PieceType::Pawn, Color::White);
        let bp = Piece::new(PieceType::Pawn, Color::Black);
        let b = Board::make_custom(vec![(wp.clone(), 0, 1), (bp.clone(), 1, 3)], Color::White);
        assert!(!b.pawn_can_move(0, 1, 1, 3));
        let b = Board::make_custom(vec![(wp.clone(), 0, 4), (bp.clone(), 1, 6)], Color::Black);
        assert!(!b.pawn_can_move(1, 6, 0, 4));
        let b = Board::make_custom(vec![(wp, 0, 1)], Color::White);
        assert!(!b.pawn_can_move(0, 1, 1, 3));
        let b = Board::make_custom(vec![(bp, 1, 6)], Color::Black);
        assert!(!b.pawn_can_move(1, 6, 0, 4));
    }

    #[test]
    fn pawn_cannot_move_backwards() {
        let wp = Piece::new(PieceType::Pawn, Color::White);
        let bp = Piece::new(PieceType::Pawn, Color::Black);
        let b = Board::make_custom(vec![(wp, 0, 1)], Color::White);
        assert!(!b.pawn_can_move(0, 1, 0, 0));
        let b = Board::make_custom(vec![(bp, 0, 6)], Color::Black);
        assert!(!b.pawn_can_move(0, 6, 0, 7));
    }

    #[test]
    fn pawn_cannnot_move_through() {
        let wp = Piece::new(PieceType::Pawn, Color::White);
        let bp = Piece::new(PieceType::Pawn, Color::Black);
        let b = Board::make_custom(vec![(wp.clone(), 0, 1), (bp.clone(), 0, 2)], Color::White);
        assert!(!b.pawn_can_move(0, 1, 0, 3));
        let b = Board::make_custom(vec![(bp, 0, 6), (wp, 0, 5)], Color::Black);
        assert!(!b.pawn_can_move(0, 6, 0, 4));
    }

    #[test]
    fn pawn_can_en_passant() {
        let wp = Piece::new(PieceType::Pawn, Color::White);
        let bp = Piece::new(PieceType::Pawn, Color::Black);

        // add kings so is_in_check doesnt panic
        let wk = Piece::new(PieceType::King, Color::White);
        let bk = Piece::new(PieceType::King, Color::Black);
        let mut b = Board::make_custom(
            vec![(wp, 0, 1), (bp, 1, 3), (wk, 0, 4), (bk, 7, 4)],
            Color::White,
        );
        assert!(b.move_piece(0, 1, 0, 3));
        assert!(b.move_piece(1, 3, 0, 2));
        assert!(b.space(0, 3).piece().is_none());
    }

    #[test]
    fn rook_can_move_into_empty() {
        let wr = Piece::new(PieceType::Rook, Color::White);
        let b = Board::make_custom(vec![(wr, 1, 1)], Color::White);
        assert!(b.rook_can_move(1, 1, 1, 4));
        assert!(b.rook_can_move(1, 1, 4, 1));
        assert!(b.rook_can_move(1, 1, 0, 1));
        assert!(b.rook_can_move(1, 1, 1, 0));
    }

    #[test]
    fn rook_cannot_move_diagonally() {
        let wr = Piece::new(PieceType::Rook, Color::White);
        let b = Board::make_custom(vec![(wr, 0, 1)], Color::White);
        assert!(!b.rook_can_move(0, 1, 1, 2));
    }

    #[test]
    fn rook_cannot_move_through() {
        let wr = Piece::new(PieceType::Rook, Color::White);
        let b = Board::make_custom(
            vec![(wr.clone(), 0, 1), (wr.clone(), 0, 3), (wr, 1, 1)],
            Color::White,
        );
        assert!(!b.rook_can_move(0, 1, 0, 5));
        assert!(!b.rook_can_move(0, 1, 2, 1));
    }

    #[test]
    fn rook_can_capture() {
        let wr = Piece::new(PieceType::Rook, Color::White);
        let br = Piece::new(PieceType::Rook, Color::Black);
        let b = Board::make_custom(
            vec![(wr.clone(), 1, 1), (br, 1, 5), (wr, 5, 1)],
            Color::White,
        );
        assert!(!b.rook_can_move(1, 1, 5, 1));
        assert!(b.rook_can_move(1, 1, 1, 5));
    }

    #[test]
    fn bishop_can_move_into_empty() {
        let wb = Piece::new(PieceType::Bishop, Color::White);
        let b = Board::make_custom(vec![(wb, 2, 3)], Color::White);
        assert!(b.bishop_can_move(2, 3, 5, 6));
        assert!(b.bishop_can_move(2, 3, 0, 5));
        assert!(b.bishop_can_move(2, 3, 4, 1));
        assert!(b.bishop_can_move(2, 3, 0, 1));
    }

    #[test]
    fn bishop_cannot_move_nondiagonally() {
        let wb = Piece::new(PieceType::Bishop, Color::White);
        let b = Board::make_custom(vec![(wb, 2, 3)], Color::White);
        assert!(!b.bishop_can_move(2, 3, 4, 6));
        assert!(!b.bishop_can_move(2, 3, 1, 5));
        assert!(!b.bishop_can_move(2, 3, 2, 1));
        assert!(!b.bishop_can_move(2, 3, 0, 0));
    }

    #[test]
    fn bishop_cannot_move_through() {
        let wb = Piece::new(PieceType::Bishop, Color::White);
        let bp = Piece::new(PieceType::Pawn, Color::Black);
        let b = Board::make_custom(vec![(wb.clone(), 2, 3), (wb.clone(), 4, 5)], Color::White);
        assert!(!b.bishop_can_move(2, 3, 5, 6));
        let b = Board::make_custom(
            vec![(wb, 7, 2), (bp.clone(), 5, 4), (bp, 4, 5)],
            Color::White,
        );
        assert!(!b.bishop_can_move(7, 2, 4, 5));
    }

    #[test]
    fn bishop_can_capture() {
        let wb = Piece::new(PieceType::Bishop, Color::White);
        let bb = Piece::new(PieceType::Bishop, Color::Black);
        let b = Board::make_custom(
            vec![(wb.clone(), 2, 3), (bb, 4, 5), (wb, 0, 5)],
            Color::White,
        );
        assert!(!b.bishop_can_move(2, 3, 5, 6));
        assert!(!b.bishop_can_move(2, 3, 0, 5));
        assert!(b.bishop_can_move(2, 3, 4, 5));
    }

    #[test]
    fn king_can_move_into_empty() {
        let wk = Piece::new(PieceType::King, Color::White);
        let b = Board::make_custom(vec![(wk, 2, 3)], Color::White);
        assert!(b.king_can_move(2, 3, 2, 4));
        assert!(b.king_can_move(2, 3, 1, 4));
        assert!(b.king_can_move(2, 3, 1, 3));
        assert!(b.king_can_move(2, 3, 3, 2));
    }

    #[test]
    fn king_cannot_move_into_check() {
        let wk = Piece::new(PieceType::King, Color::White);
        let bq = Piece::new(PieceType::Queen, Color::Black);
        let b = Board::make_custom(vec![(wk, 2, 3), (bq, 5, 4)], Color::White);
        assert!(!b.king_can_move(2, 3, 2, 4));
        assert!(!b.king_can_move(2, 3, 1, 4));
        assert!(b.king_can_move(2, 3, 1, 3));
        assert!(!b.king_can_move(2, 3, 3, 2));
    }

    #[test]
    fn king_can_capture() {
        let wk = Piece::new(PieceType::King, Color::White);
        let bk = Piece::new(PieceType::King, Color::Black);
        let b = Board::make_custom(
            vec![(wk.clone(), 2, 3), (wk, 2, 4), (bk, 1, 4)],
            Color::White,
        );
        assert!(!b.king_can_move(2, 3, 2, 4));
        assert!(b.king_can_move(2, 3, 1, 4));
    }

    #[test]
    fn knight_can_move_into_empty() {
        let wn = Piece::new(PieceType::Knight, Color::White);
        let b = Board::make_custom(vec![(wn, 6, 2)], Color::White);
        assert!(b.knight_can_move(6, 2, 5, 4));
        assert!(b.knight_can_move(6, 2, 7, 4));
        assert!(b.knight_can_move(6, 2, 4, 3));
        assert!(b.knight_can_move(6, 2, 5, 0));
        assert!(b.knight_can_move(6, 2, 7, 0));
    }

    #[test]
    fn knight_can_capture() {
        let wn = Piece::new(PieceType::Knight, Color::White);
        let bn = Piece::new(PieceType::Knight, Color::Black);
        let b = Board::make_custom(
            vec![(wn.clone(), 6, 2), (wn, 5, 4), (bn, 7, 4)],
            Color::White,
        );
        assert!(!b.knight_can_move(6, 2, 5, 4));
        assert!(b.knight_can_move(6, 2, 7, 4));
    }

    #[test]
    fn space_is_attacked() {
        let wq = Piece::new(PieceType::Queen, Color::White);
        let bq = Piece::new(PieceType::Queen, Color::Black);
        let b = Board::make_custom(vec![(wq, 3, 4), (bq, 3, 5)], Color::White);
        assert!(b.is_space_attacked(0, 4, Color::Black));
        assert!(!b.is_space_attacked(0, 4, Color::White));
        assert!(!b.is_space_attacked(3, 6, Color::Black));
        assert!(b.is_space_attacked(3, 6, Color::White));
        assert!(b.is_space_attacked(1, 7, Color::White));
    }

    #[test]
    fn queenside_castle() {
        let wk = Piece::new(PieceType::King, Color::White);
        let wr = Piece::new(PieceType::Rook, Color::White);
        let mut b = Board::make_custom(vec![(wk, 4, 0), (wr, 0, 0)], Color::White);
        assert!(b.move_piece(4, 0, 2, 0));
        assert_eq!(
            b.space(2, 0).piece().as_ref().unwrap().piece_type(),
            PieceType::King
        );
        assert_eq!(
            b.space(3, 0).piece().as_ref().unwrap().piece_type(),
            PieceType::Rook
        );
        let bk = Piece::new(PieceType::King, Color::Black);
        let br = Piece::new(PieceType::Rook, Color::Black);
        let mut b = Board::make_custom(vec![(bk, 4, 7), (br, 0, 7)], Color::Black);
        assert!(b.move_piece(4, 7, 2, 7));
        assert_eq!(
            b.space(2, 7).piece().as_ref().unwrap().piece_type(),
            PieceType::King
        );
        assert_eq!(
            b.space(3, 7).piece().as_ref().unwrap().piece_type(),
            PieceType::Rook
        );
    }

    #[test]
    fn kingside_castle() {
        let wk = Piece::new(PieceType::King, Color::White);
        let wr = Piece::new(PieceType::Rook, Color::White);
        let mut b = Board::make_custom(vec![(wk, 4, 0), (wr, 7, 0)], Color::White);
        assert!(b.move_piece(4, 0, 6, 0));
        assert_eq!(
            b.space(6, 0).piece().as_ref().unwrap().piece_type(),
            PieceType::King
        );
        assert_eq!(
            b.space(5, 0).piece().as_ref().unwrap().piece_type(),
            PieceType::Rook
        );
        let bk = Piece::new(PieceType::King, Color::Black);
        let br = Piece::new(PieceType::Rook, Color::Black);
        let mut b = Board::make_custom(vec![(bk, 4, 7), (br, 7, 7)], Color::Black);
        assert!(b.move_piece(4, 7, 6, 7));
        assert_eq!(
            b.space(6, 7).piece().as_ref().unwrap().piece_type(),
            PieceType::King
        );
        assert_eq!(
            b.space(5, 7).piece().as_ref().unwrap().piece_type(),
            PieceType::Rook
        );
    }

    #[test]
    fn cant_castle_through_check() {
        let wk = Piece::new(PieceType::King, Color::White);
        let wr = Piece::new(PieceType::Rook, Color::White);
        let br = Piece::new(PieceType::Rook, Color::Black);
        let mut b = Board::make_custom(
            vec![(wk, 4, 0), (wr.clone(), 7, 0), (br.clone(), 5, 2)],
            Color::White,
        );
        assert!(!b.move_piece(4, 0, 6, 0));
        let bk = Piece::new(PieceType::King, Color::Black);
        let mut b = Board::make_custom(vec![(bk, 4, 7), (br, 7, 7), (wr, 5, 5)], Color::White);
        assert!(!b.move_piece(4, 7, 6, 7));
    }

    #[test]
    fn board_from_strs_default() {
        let b = Board::new();
        let strs = vec![
            "rnbqkbnr", "pppppppp", "________", "________", "________", "________", "PPPPPPPP",
            "RNBQKBNR", "W",
        ];
        let b2 = Board::from_strs(&strs);
        assert_eq!(b, b2);
    }

    #[test]
    fn undo_first_move() {
        let mut b = Board::new();
        let b2 = Board::new();
        assert!(b.move_piece(1, 1, 1, 3));
        assert_ne!(b, b2);
        assert_eq!(b.turn_color, Color::Black);
        b.undo_last_move();
        assert_eq!(b, b2);
        assert_eq!(b.turn_color, Color::White);
    }

    #[test]
    fn undo_capture() {
        let mut b = Board::new();
        let mut b2 = Board::new();
        assert!(b.move_piece(1, 1, 1, 3));
        assert!(b2.move_piece(1, 1, 1, 3));
        assert!(b.move_piece(2, 6, 2, 4));
        assert!(b2.move_piece(2, 6, 2, 4));
        assert!(b.move_piece(1, 3, 2, 4));
        assert_ne!(b, b2);
        assert_eq!(b.turn_color, Color::Black);
        b.undo_last_move();
        assert_eq!(b, b2);
        assert_eq!(b.turn_color, Color::White);
    }

    #[test]
    fn undo_castle() {
        let wk = Piece::new(PieceType::King, Color::White);
        let wr = Piece::new(PieceType::Rook, Color::White);
        let mut b = Board::make_custom(vec![(wk, 4, 0), (wr, 7, 0)], Color::White);
        let b2 = b.clone();
        assert!(b.move_piece(4, 0, 6, 0));
        assert_eq!(b.turn_color, Color::Black);
        b.undo_last_move();
        assert_eq!(b, b2);
        assert_eq!(b.turn_color, Color::White);

        let bk = Piece::new(PieceType::King, Color::Black);
        let br = Piece::new(PieceType::Rook, Color::Black);
        let mut b = Board::make_custom(vec![(bk, 4, 7), (br, 0, 7)], Color::Black);
        let b2 = b.clone();
        assert!(b.move_piece(4, 7, 2, 7));
        assert_eq!(b.turn_color, Color::White);
        b.undo_last_move();
        assert_eq!(b, b2);
        assert_eq!(b.turn_color, Color::Black);
    }

    #[test]
    fn prevent_move_exposing_check() {
        let mut b = Board::new();
        assert!(b.move_piece(4, 1, 4, 2));
        assert!(b.move_piece(2, 6, 2, 5));
        assert!(b.move_piece(4, 2, 4, 3));
        assert!(b.move_piece(3, 7, 0, 4));
        let b2 = b.clone();
        assert!(!b.move_piece(3, 1, 3, 2)); // cannot move this pawn as it would expose king to check from queen
        assert_eq!(b, b2);
    }

    #[test]
    fn promote() {
        let wp = Piece::new(PieceType::Pawn, Color::White);
        let bp = Piece::new(PieceType::Pawn, Color::Black);
        let mut b = Board::make_custom(vec![(wp, 0, 7), (bp, 0, 0)], Color::White);
        let wb = Piece::new(PieceType::Bishop, Color::White);
        let br = Piece::new(PieceType::Rook, Color::Black);
        let b2 = Board::make_custom(vec![(wb, 0, 7), (br, 0, 0)], Color::White);
        b.promote_pawn(0, 7, PieceType::Bishop);
        b.promote_pawn(0, 0, PieceType::Rook);
        assert_eq!(b, b2);
    }

    #[test]
    fn record_capture() {
        let wr = Piece::new(PieceType::Rook, Color::White);
        let br = Piece::new(PieceType::Rook, Color::Black);
        let wp = Piece::new(PieceType::Pawn, Color::White);
        let bp = Piece::new(PieceType::Pawn, Color::Black);
        // add kings so is_in_check doesnt panic
        let wk = Piece::new(PieceType::King, Color::White);
        let bk = Piece::new(PieceType::King, Color::Black);
        let mut b = Board::make_custom(
            vec![
                (wr, 0, 0),
                (br, 2, 1),
                (wp, 1, 1),
                (bp.clone(), 0, 1),
                (bp, 1, 4),
                (wk, 7, 7),
                (bk, 5, 7),
            ],
            Color::White,
        );
        assert!(b.captured_by_white.is_empty());
        assert!(b.captured_by_black.is_empty());
        assert!(b.move_piece(0, 0, 0, 1));
        assert_eq!(b.captured_by_white.get(&PieceType::Pawn).unwrap(), &1);
        assert!(b.captured_by_black.is_empty());
        assert!(b.move_piece(2, 1, 1, 1));
        assert_eq!(b.captured_by_black.get(&PieceType::Pawn).unwrap(), &1);
        assert!(b.move_piece(0, 1, 1, 1));
        assert_eq!(b.captured_by_white.get(&PieceType::Rook).unwrap(), &1);
        assert!(b.move_piece(1, 4, 1, 3));
        assert!(b.move_piece(1, 1, 1, 3));
        assert_eq!(b.captured_by_white.get(&PieceType::Pawn).unwrap(), &2);
        b.undo_last_move();
        assert_eq!(b.captured_by_white.get(&PieceType::Pawn).unwrap(), &1);
        b.undo_last_move();
        b.undo_last_move();
        assert!(!b.captured_by_white.contains_key(&PieceType::Rook));
    }
}

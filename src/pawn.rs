use crate::color::Color;
use crate::piece::Piece;

pub struct Pawn {
    color: Color,
    has_moved: bool,
}

impl Pawn {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            has_moved: false,
        }
    }
}

impl Piece for Pawn {
    fn color(&self) -> Color {
        self.color
    }

    fn can_move(&self, x1: u8, y1: u8, x2: u8, y2: u8, piece2: &Option<Box<dyn Piece>>) -> bool {
        match self.color {
            Color::White => {
                (!self.has_moved && y1 + 2 == y2 && x1 == x2 && piece2.is_none())
                    || (y1 + 1 == y2
                        && ((x1 == x2 && piece2.is_none())
                            || ((x1 + 1 == x2 || x2 + 1 == x1)
                                && piece2.is_some()
                                && piece2.as_ref().unwrap().color() == Color::Black)))
            }
            Color::Black => {
                (!self.has_moved && y2 + 2 == y1 && x1 == x2 && piece2.is_none())
                    || (y2 + 1 == y1
                        && ((x1 == x2 && piece2.is_none())
                            || ((x1 + 1 == x2 || x2 + 1 == x1)
                                && piece2.is_some()
                                && piece2.as_ref().unwrap().color() == Color::White)))
            }
        }
    }

    fn draw(&self) -> char {
        'P'
    }

    fn mark_moved(&mut self) {
        self.has_moved = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_move_forward_into_empty() {
        let wp = Pawn::new(Color::White);
        let bp = Pawn::new(Color::Black);
        assert_eq!(wp.can_move(0, 1, 0, 2, &None), true);
        assert_eq!(bp.can_move(0, 6, 0, 5, &None), true);
    }

    #[test]
    fn can_attack() {
        let wp = Pawn::new(Color::White);
        let bp = Pawn::new(Color::Black);
        let wp_op: Option<Box<dyn Piece>> = Some(Box::new(Pawn::new(Color::White)));
        let bp_op: Option<Box<dyn Piece>> = Some(Box::new(Pawn::new(Color::Black)));
        assert_eq!(wp.can_move(0, 1, 1, 2, &bp_op), true);
        assert_eq!(bp.can_move(1, 6, 0, 5, &wp_op), true);
    }

    #[test]
    fn cannot_move_forward_into_occupied() {
        let wp = Pawn::new(Color::White);
        let bp = Pawn::new(Color::Black);
        let wp_op: Option<Box<dyn Piece>> = Some(Box::new(Pawn::new(Color::White)));
        let bp_op: Option<Box<dyn Piece>> = Some(Box::new(Pawn::new(Color::Black)));
        assert_eq!(wp.can_move(0, 1, 0, 2, &wp_op), false);
        assert_eq!(bp.can_move(0, 6, 0, 5, &bp_op), false);
        assert_eq!(wp.can_move(0, 1, 0, 3, &wp_op), false);
        assert_eq!(bp.can_move(0, 6, 0, 4, &bp_op), false);
    }

    #[test]
    fn can_move_forward_two() {
        let wp = Pawn::new(Color::White);
        let bp = Pawn::new(Color::Black);
        assert_eq!(wp.can_move(0, 1, 0, 3, &None), true);
        assert_eq!(bp.can_move(0, 6, 0, 4, &None), true);
    }

    #[test]
    fn cannot_move_forward_two_after_move() {
        let mut wp = Pawn::new(Color::White);
        let mut bp = Pawn::new(Color::Black);
        wp.mark_moved();
        bp.mark_moved();
        assert_eq!(wp.can_move(0, 1, 0, 3, &None), false);
        assert_eq!(bp.can_move(0, 6, 0, 4, &None), false);
    }

    #[test]
    fn cannot_attack_into_empty() {
        let wp = Pawn::new(Color::White);
        let bp = Pawn::new(Color::Black);
        assert_eq!(wp.can_move(0, 1, 1, 2, &None), false);
        assert_eq!(bp.can_move(1, 6, 0, 5, &None), false);
    }

    #[test]
    fn cannot_move_two_and_attack() {
        let wp = Pawn::new(Color::White);
        let bp = Pawn::new(Color::Black);
        let wp_op: Option<Box<dyn Piece>> = Some(Box::new(Pawn::new(Color::White)));
        let bp_op: Option<Box<dyn Piece>> = Some(Box::new(Pawn::new(Color::Black)));
        assert_eq!(wp.can_move(0, 1, 1, 3, &wp_op), false);
        assert_eq!(bp.can_move(1, 6, 0, 4, &bp_op), false);
        assert_eq!(wp.can_move(0, 1, 1, 3, &None), false);
        assert_eq!(bp.can_move(1, 6, 0, 4, &None), false);
    }

    #[test]
    fn cannot_move_backwards() {
        let wp = Pawn::new(Color::White);
        let bp = Pawn::new(Color::Black);
        assert_eq!(wp.can_move(0, 1, 0, 0, &None), false);
        assert_eq!(bp.can_move(0, 6, 0, 7, &None), false);
    }
}

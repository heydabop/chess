use crate::color::Color;

pub trait Piece {
    fn color(&self) -> Color;
    fn can_move(&self, x1: u8, y1: u8, x2: u8, y2: u8, piece2: &Option<Box<dyn Piece>>) -> bool;
    fn draw(&self) -> char;
    fn mark_moved(&mut self);
    fn piece_type(&self) -> PieceType;
}

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn,
}

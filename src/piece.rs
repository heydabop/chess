use crate::color::Color;

pub trait Piece {
    fn color(&self) -> Color;
    fn can_move(&self, x1: u8, y1: u8, x2: u8, y2: u8, piece2: &Option<Box<dyn Piece>>) -> bool;
    fn draw(&self) -> char;
}

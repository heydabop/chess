mod bishop;
mod king;
mod knight;
mod pawn;
mod piece_type;
mod queen;
mod rook;

pub use self::bishop::Bishop;
pub use self::king::King;
pub use self::knight::Knight;
pub use self::pawn::Pawn;
pub use self::piece_type::PieceType;
pub use self::queen::Queen;
pub use self::rook::Rook;
use crate::color::Color;

pub trait Piece {
    fn color(&self) -> Color;
    fn can_move(&self, x1: u8, y1: u8, x2: u8, y2: u8, piece2: &Option<Box<dyn Piece>>) -> bool;
    fn draw(&self) -> char;
    fn mark_moved(&mut self);
    fn piece_type(&self) -> PieceType;
}

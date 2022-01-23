#![feature(array_from_fn)]
#![allow(dead_code)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::too_many_lines,
    clippy::similar_names
)]

mod board;
mod color;
mod game;
mod move_record;
mod piece;
mod space;

use crossterm::Result;
use game::Game;

fn main() -> Result<()> {
    let mut game = Game::new();

    game.run_loop()?;

    Ok(())
}

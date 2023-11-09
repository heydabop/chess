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
mod netplay;
mod piece;
mod space;

use game::Game;
use std::io::Result;

fn main() -> Result<()> {
    let netplay = netplay::NetPlay::init_tcp_from_stdin()?;

    let mut game = if let Some(netplay) = netplay {
        Game::with_tcp_netplay(netplay)
    } else {
        Game::new()
    };

    game.run_loop()?;

    Ok(())
}

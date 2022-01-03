#![feature(array_from_fn)]
#![allow(dead_code)]
#![warn(clippy::pedantic)]

mod board;
mod color;
mod pawn;
mod piece;
mod space;

use board::Board;
use crossterm::{
    cursor,
    event::read,
    execute, queue,
    style::{self, Stylize},
    terminal, Result,
};
use std::io::{stdout, Write};

fn main() -> Result<()> {
    let board = Board::new();

    let mut stdout = stdout();

    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    let spaces = board.spaces();
    for y in 0u8..8u8 {
        for x in 0u8..8u8 {
            queue!(
                stdout,
                cursor::MoveTo(x.into(), y.into()),
                style::PrintStyledContent(spaces[y as usize][x as usize].draw().magenta())
            )?;
        }
    }
    stdout.flush()?;
    read()?;
    Ok(())
}

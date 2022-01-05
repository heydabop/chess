#![feature(array_from_fn)]
#![allow(dead_code)]
#![warn(clippy::pedantic)]

mod board;
mod color;
mod pawn;
mod piece;
mod space;

use board::Board;
use color::Color;
use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
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
                style::PrintStyledContent(spaces[y as usize][x as usize].draw().green())
            )?;
        }
    }
    queue!(
        stdout,
        cursor::SetCursorShape(cursor::CursorShape::Block),
        cursor::EnableBlinking,
        cursor::Show,
    )?;
    stdout.flush()?;
    terminal::enable_raw_mode()?;
    loop {
        let e = read()?;
        if let Event::Key(k) = e {
            match k.code {
                KeyCode::Up => execute!(stdout, cursor::MoveUp(1)),
                KeyCode::Down => execute!(stdout, cursor::MoveDown(1)),
                KeyCode::Left => execute!(stdout, cursor::MoveLeft(1)),
                KeyCode::Right => execute!(stdout, cursor::MoveRight(1)),
                KeyCode::Char('q') => break,
                _ => Ok(()),
            }?;
        }
    }
    terminal::disable_raw_mode()?;
    Ok(())
}

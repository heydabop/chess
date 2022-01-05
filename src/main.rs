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
    style::{self, Color as TermColor, Stylize},
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
            let space = &spaces[7 - y as usize][x as usize];
            let space_color = match space.color() {
                Color::White => TermColor::White,
                Color::Black => TermColor::Black,
            };
            let piece_color = match space.piece_color() {
                Some(Color::White) => TermColor::Grey,
                Some(Color::Black) => TermColor::DarkGrey,
                None => TermColor::White,
            };
            queue!(
                stdout,
                cursor::MoveTo(x.into(), y.into()),
                style::PrintStyledContent(space.draw().with(piece_color).on(space_color))
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

    game_loop(board)?;

    terminal::disable_raw_mode()?;
    execute!(stdout, cursor::MoveTo(0, 0))?;

    Ok(())
}

fn game_loop(mut board: Board) -> Result<()> {
    let mut stdout = stdout();

    loop {
        let e = read()?;
        if let Event::Key(k) = e {
            match k.code {
                KeyCode::Up => execute!(stdout, cursor::MoveUp(1))?,
                KeyCode::Down => execute!(stdout, cursor::MoveDown(1))?,
                KeyCode::Left => execute!(stdout, cursor::MoveLeft(1))?,
                KeyCode::Right => execute!(stdout, cursor::MoveRight(1))?,
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char(' ') => {
                    let pos = cursor::position()?;
                    let x = pos.0 as usize;
                    let y = pos.1 as usize;
                    if x < 8 && y < 8 {
                        let space = &board.spaces()[7 - y][x];
                        if let Some(piece_color) = space.piece_color() {
                            if piece_color == board.turn_color() {
                                execute!(
                                    stdout,
                                    style::PrintStyledContent(space.draw().black().on_green()),
                                    cursor::MoveLeft(1)
                                )?;
                                board.next_turn();
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

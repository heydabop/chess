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
    let turn_color = Color::White;
    let last = loop {
        match read() {
            Ok(e) => {
                if let Event::Key(k) = e {
                    if let Err(ke) = match k.code {
                        KeyCode::Up => execute!(stdout, cursor::MoveUp(1)),
                        KeyCode::Down => execute!(stdout, cursor::MoveDown(1)),
                        KeyCode::Left => execute!(stdout, cursor::MoveLeft(1)),
                        KeyCode::Right => execute!(stdout, cursor::MoveRight(1)),
                        KeyCode::Char('q') => break Ok(()),
                        KeyCode::Char(' ') => {
                            let pos = match cursor::position() {
                                Ok(p) => p,
                                Err(e) => break Err(e),
                            };
                            let x = pos.0 as usize;
                            let y = pos.1 as usize;
                            if x < 8 && y < 8 {
                                let space = &spaces[7 - y][x];
                                if let Some(piece_color) = space.piece_color() {
                                    if piece_color == turn_color {
                                        execute!(
                                            stdout,
                                            style::PrintStyledContent(
                                                space.draw().black().on_green()
                                            ),
                                            cursor::MoveLeft(1)
                                        )
                                    } else {
                                        Ok(())
                                    }
                                } else {
                                    Ok(())
                                }
                            } else {
                                Ok(())
                            }
                        }
                        _ => Ok(()),
                    } {
                        break Err(ke);
                    }
                }
            }
            Err(e) => break Err(e),
        };
    };
    // If last is an err these commands are probably also going to fail...
    terminal::disable_raw_mode()?;
    execute!(stdout, cursor::MoveTo(0, 0))?;

    last
}

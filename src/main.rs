#![feature(array_from_fn)]
#![allow(dead_code)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::too_many_lines)]

mod board;
mod color;
mod move_record;
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
use space::Space;
use std::io::{stdout, Write};

fn main() -> Result<()> {
    let board = Board::new();

    let mut stdout = stdout();

    queue!(stdout, terminal::Clear(terminal::ClearType::All))?;
    queue_board(&board)?;
    queue!(
        stdout,
        cursor::MoveLeft(1),
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
    let mut selected: Option<(u8, u8)> = None;
    let mut undoing = false;
    let mut quitting = false;

    loop {
        let e = read()?;
        if let Event::Key(k) = e {
            let pos = cursor::position()?;
            match k.code {
                KeyCode::Up => execute!(stdout, cursor::MoveUp(1))?,
                KeyCode::Down => {
                    if pos.1 < 7 {
                        execute!(stdout, cursor::MoveDown(1))?;
                    }
                }
                KeyCode::Left => execute!(stdout, cursor::MoveLeft(1))?,
                KeyCode::Right => {
                    if pos.0 < 7 {
                        execute!(stdout, cursor::MoveRight(1))?;
                    }
                }
                KeyCode::Char('z') => undoing = !undoing,
                KeyCode::Char('q') => quitting = !quitting,
                KeyCode::Char('y') => {
                    if undoing {
                        board.undo_last_move();
                        undoing = false;
                        queue_board(&board)?;
                        #[allow(clippy::cast_possible_truncation)]
                        let x = pos.0 as u8;
                        #[allow(clippy::cast_possible_truncation)]
                        let y = (7 - pos.1) as u8;
                        queue!(stdout, cursor::MoveTo(x.into(), (7 - y).into()))?;
                        stdout.flush()?;
                    }
                    if quitting {
                        return Ok(());
                    }
                }
                KeyCode::Char('n') => {
                    quitting = false;
                    undoing = false;
                }
                KeyCode::Char(' ') => {
                    if pos.0 > 7 || pos.1 > 7 {
                        continue;
                    }
                    #[allow(clippy::cast_possible_truncation)]
                    let x = pos.0 as u8;
                    #[allow(clippy::cast_possible_truncation)]
                    let y = (7 - pos.1) as u8;
                    if let Some(s) = selected {
                        if s.0 == x && s.1 == y {
                            let space = board.space(x, y);
                            let colors = get_term_colors(space);
                            execute!(
                                stdout,
                                style::PrintStyledContent(space.draw().with(colors.0).on(colors.1)),
                                cursor::MoveLeft(1)
                            )?;
                            selected = None;
                        } else if x < 8 && y < 8 && board.move_piece(s.0, s.1, x, y) {
                            selected = None;
                            board.next_turn();

                            queue_board(&board)?;
                            queue!(stdout, cursor::MoveTo(x.into(), (7 - y).into()))?;
                            stdout.flush()?;
                        }
                    } else if x < 8 && y < 8 {
                        let space = board.space(x, y);
                        if let Some(piece_color) = space.piece_color() {
                            if piece_color == board.turn_color() {
                                let colors = get_term_colors(space);
                                execute!(
                                    stdout,
                                    style::PrintStyledContent(
                                        space.draw().with(colors.0).on_green()
                                    ),
                                    cursor::MoveLeft(1)
                                )?;
                                selected = Some((x, y));
                            }
                        }
                    }
                }
                _ => {}
            };
        }
    }
}

fn get_term_colors(space: &Space) -> (TermColor, TermColor) {
    let piece_color = match space.piece_color() {
        Some(Color::White) => TermColor::Grey,
        Some(Color::Black) => TermColor::DarkGrey,
        None => TermColor::White,
    };
    let space_color = match space.color() {
        Color::White => TermColor::White,
        Color::Black => TermColor::Black,
    };
    (piece_color, space_color)
}

fn queue_board(board: &Board) -> Result<()> {
    let mut stdout = stdout();
    for y in 0u8..8u8 {
        for x in 0u8..8u8 {
            let space = board.space(x, 7 - y);
            let colors = get_term_colors(space);
            queue!(
                stdout,
                cursor::MoveTo(x.into(), y.into()),
                style::PrintStyledContent(space.draw().with(colors.0).on(colors.1))
            )?;
        }
    }

    Ok(())
}

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

const SPACE_WIDTH: u16 = 5;
const SPACE_HEIGHT: u16 = 3;
const MIN_X: u16 = SPACE_WIDTH / 2;
const MAX_X: u16 = SPACE_WIDTH * 7 - MIN_X;
const MIN_Y: u16 = SPACE_HEIGHT / 2;
const MAX_Y: u16 = SPACE_HEIGHT * 7 + MIN_Y;

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
        cursor::MoveTo(SPACE_WIDTH / 2, SPACE_HEIGHT * 7 + SPACE_HEIGHT / 2),
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
                KeyCode::Up => {
                    if pos.1 > MIN_Y {
                        execute!(stdout, cursor::MoveUp(SPACE_HEIGHT))?;
                    }
                }
                KeyCode::Down => {
                    if pos.1 < MAX_Y {
                        execute!(stdout, cursor::MoveDown(SPACE_HEIGHT))?;
                    }
                }
                KeyCode::Left => {
                    if pos.0 > MIN_X {
                        execute!(stdout, cursor::MoveLeft(SPACE_WIDTH))?;
                    }
                }
                KeyCode::Right => {
                    if pos.0 < MAX_X {
                        execute!(stdout, cursor::MoveRight(SPACE_WIDTH))?;
                    }
                }
                KeyCode::Char('z') => {
                    undoing = true;
                    quitting = false;
                }
                KeyCode::Char('q') => {
                    quitting = true;
                    undoing = false;
                }
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
                    if pos.0 > MAX_X || pos.1 > MAX_Y {
                        continue;
                    }
                    #[allow(clippy::cast_possible_truncation)]
                    let x = (pos.0 / SPACE_WIDTH) as u8;
                    #[allow(clippy::cast_possible_truncation)]
                    let y = 7 - (pos.1 / SPACE_HEIGHT) as u8;
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
                            queue!(
                                stdout,
                                cursor::MoveTo(
                                    u16::from(x) * SPACE_WIDTH + (SPACE_WIDTH / 2),
                                    (7 - u16::from(y)) * SPACE_HEIGHT + (SPACE_HEIGHT / 2)
                                )
                            )?;
                            stdout.flush()?;
                        }
                    } else if x < 8 && y < 8 {
                        let space = board.space(x, y);
                        if let Some(piece_color) = space.piece_color() {
                            if piece_color == board.turn_color() {
                                queue_space(space, x, y, true)?;
                                queue!(stdout, cursor::MoveUp(MIN_Y), cursor::MoveLeft(MIN_X + 1))?;
                                stdout.flush()?;
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
    for y in 0u8..8u8 {
        for x in 0u8..8u8 {
            let space = board.space(x, 7 - y);
            queue_space(space, x, 7 - y, false)?;
        }
    }

    Ok(())
}

fn queue_space(space: &Space, space_x: u8, space_y: u8, highlighted: bool) -> Result<()> {
    let x = u16::from(space_x) * SPACE_WIDTH;
    let y = (7 - u16::from(space_y)) * SPACE_HEIGHT;
    let mut stdout = stdout();
    let (fg_color, bg_color) = get_term_colors(space);
    queue!(
        stdout,
        cursor::MoveTo(x, y),
        style::PrintStyledContent(' '.on(bg_color)),
        style::PrintStyledContent(' '.on(bg_color)),
        style::PrintStyledContent(' '.on(bg_color)),
        style::PrintStyledContent(' '.on(bg_color)),
        style::PrintStyledContent(' '.on(bg_color)),
        cursor::MoveTo(x, y + 1),
        style::PrintStyledContent(' '.on(bg_color)),
        style::PrintStyledContent(' '.on(bg_color)),
        if highlighted {
            style::PrintStyledContent(space.draw().with(fg_color).on_green())
        } else if space.piece().is_some() {
            style::PrintStyledContent(space.draw().with(fg_color).on_black())
        } else {
            style::PrintStyledContent(space.draw().with(fg_color).on(bg_color))
        },
        style::PrintStyledContent(' '.on(bg_color)),
        style::PrintStyledContent(' '.on(bg_color)),
        cursor::MoveTo(x, y + 2),
        style::PrintStyledContent(' '.on(bg_color)),
        style::PrintStyledContent(' '.on(bg_color)),
        style::PrintStyledContent(' '.on(bg_color)),
        style::PrintStyledContent(' '.on(bg_color)),
        style::PrintStyledContent(' '.on(bg_color)),
    )?;

    Ok(())
}

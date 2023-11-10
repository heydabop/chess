use crate::color::Color;
use crate::netplay::Command;
use crate::piece::PieceType;
use crate::space::Space;
use crate::{board::Board, netplay::NetPlay};
use crossterm::event::KeyEvent;
use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    execute, queue,
    style::{self, Color as TermColor, Stylize},
    terminal,
};
use std::collections::HashMap;
use std::io::{stdout, Result, Stdout, Write};
use std::net::TcpStream;

const SPACE_WIDTH: u16 = 5;
const SPACE_HEIGHT: u16 = 3;
const MIN_X: u16 = SPACE_WIDTH / 2;
const MAX_X: u16 = SPACE_WIDTH * 7 + MIN_X;
const MIN_Y: u16 = SPACE_HEIGHT / 2;
const MAX_Y: u16 = SPACE_HEIGHT * 7 + MIN_Y;

pub struct Game {
    board: Board,
    selected: Option<(u8, u8)>,
    undoing: bool,
    quitting: bool,
    promoting: Option<(u8, u8)>,
    stdout: Stdout,
    victor: Option<Color>,
    netplay: Option<NetPlay<TcpStream>>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            selected: None,
            undoing: false,
            quitting: false,
            promoting: None,
            stdout: stdout(),
            victor: None,
            netplay: None,
        }
    }

    pub fn with_board(board: Board) -> Self {
        Self {
            board,
            selected: None,
            undoing: false,
            quitting: false,
            promoting: None,
            stdout: stdout(),
            victor: None,
            netplay: None,
        }
    }

    pub fn with_tcp_netplay(netplay: NetPlay<TcpStream>) -> Self {
        Self {
            board: Board::new(),
            selected: None,
            undoing: false,
            quitting: false,
            promoting: None,
            stdout: stdout(),
            victor: None,
            netplay: Some(netplay),
        }
    }

    pub fn run_loop(&mut self) -> Result<()> {
        queue!(self.stdout, terminal::Clear(terminal::ClearType::All))?;
        self.queue_board()?;
        self.queue_captured_pieces()?;
        queue!(
            self.stdout,
            cursor::MoveLeft(1),
            cursor::SetCursorStyle::BlinkingBlock,
            cursor::Show,
            cursor::MoveTo(SPACE_WIDTH / 2, SPACE_HEIGHT * 7 + SPACE_HEIGHT / 2),
        )?;
        self.queue_status_text()?;
        self.stdout.flush()?;
        terminal::enable_raw_mode()?;

        loop {
            if let Some(ref mut netplay) = self.netplay {
                if netplay.is_host() && self.board.turn_color() == Color::White
                    || !netplay.is_host() && self.board.turn_color() == Color::Black
                {
                    let e = read()?;
                    if let Event::Key(k) = e {
                        if self.handle_key_event(k)? {
                            break;
                        }
                    }
                } else {
                    match netplay.recv()? {
                        Command::Ack => panic!("unexpected ack"),
                        Command::Move { x0, y0, x1, y1 } => {
                            assert!(
                                self.board.move_piece(x0, y0, x1, y1),
                                "unable to move piece {x0} {y0} {x1} {y1}"
                            );
                        }
                        Command::Promote { x, y, piece } => {
                            self.board.promote_pawn(x, y, piece);
                        }
                    }
                    self.check_victor();
                    self.queue_board()?;
                    self.queue_captured_pieces()?;
                    queue!(
                        self.stdout,
                        cursor::MoveTo(SPACE_WIDTH / 2, 7 * SPACE_HEIGHT + (SPACE_HEIGHT / 2))
                    )?;
                    self.queue_status_text()?;
                    self.stdout.flush()?;
                }
            } else {
                let e = read()?;
                if let Event::Key(k) = e {
                    if self.handle_key_event(k)? {
                        break;
                    }
                }
            }
        }

        terminal::disable_raw_mode()?;
        execute!(self.stdout, cursor::MoveTo(0, 0))?;

        Ok(())
    }

    // returns true if quitting
    fn handle_key_event(&mut self, k: KeyEvent) -> Result<bool> {
        let pos = cursor::position()?;
        let can_move = if let Some(netplay) = &self.netplay {
            (netplay.is_host() && self.board.turn_color() == Color::White
                || !netplay.is_host() && self.board.turn_color() == Color::Black)
                && self.promoting.is_none()
                && self.victor.is_none()
        } else {
            self.promoting.is_none() && self.victor.is_none()
        };
        match k.code {
            KeyCode::Up => {
                if can_move && pos.1 > MIN_Y {
                    execute!(self.stdout, cursor::MoveUp(SPACE_HEIGHT))?;
                }
            }
            KeyCode::Down => {
                if can_move && pos.1 < MAX_Y {
                    execute!(self.stdout, cursor::MoveDown(SPACE_HEIGHT))?;
                }
            }
            KeyCode::Left => {
                if can_move && pos.0 > MIN_X {
                    execute!(self.stdout, cursor::MoveLeft(SPACE_WIDTH))?;
                }
            }
            KeyCode::Right => {
                if can_move && pos.0 < MAX_X {
                    execute!(self.stdout, cursor::MoveRight(SPACE_WIDTH))?;
                }
            }
            // promote to bishop
            KeyCode::Char('b') => {
                if let Some(promoting) = self.promoting {
                    self.board
                        .promote_pawn(promoting.0, promoting.1, PieceType::Bishop);
                    if let Some(ref mut netplay) = self.netplay {
                        netplay.send(Command::Promote {
                            x: promoting.0,
                            y: promoting.1,
                            piece: PieceType::Bishop,
                        })?;
                    }
                    self.promoting = None;
                    self.check_victor();
                    self.queue_board()?;
                    self.queue_status_text()?;
                    queue!(self.stdout, cursor::MoveTo(pos.0, pos.1))?;
                    self.stdout.flush()?;
                }
            }
            // promote to rook
            KeyCode::Char('r') => {
                if let Some(promoting) = self.promoting {
                    self.board
                        .promote_pawn(promoting.0, promoting.1, PieceType::Rook);
                    if let Some(ref mut netplay) = self.netplay {
                        netplay.send(Command::Promote {
                            x: promoting.0,
                            y: promoting.1,
                            piece: PieceType::Pawn,
                        })?;
                    }
                    self.promoting = None;
                    self.check_victor();
                    self.queue_board()?;
                    self.queue_status_text()?;
                    queue!(self.stdout, cursor::MoveTo(pos.0, pos.1))?;
                    self.stdout.flush()?;
                }
            }
            // prompt to undo
            KeyCode::Char('z' | 'u') => {
                if self.netplay.is_none() {
                    self.undoing = true;
                    self.quitting = false;
                    self.queue_status_text()?;
                    self.stdout.flush()?;
                }
            }
            // prompt to quit or promote to queen
            KeyCode::Char('q') => {
                if let Some(promoting) = self.promoting {
                    self.board
                        .promote_pawn(promoting.0, promoting.1, PieceType::Queen);
                    if let Some(ref mut netplay) = self.netplay {
                        netplay.send(Command::Promote {
                            x: promoting.0,
                            y: promoting.1,
                            piece: PieceType::Queen,
                        })?;
                    }
                    self.promoting = None;
                    self.check_victor();
                    self.queue_board()?;
                    self.queue_status_text()?;
                    queue!(self.stdout, cursor::MoveTo(pos.0, pos.1))?;
                    self.stdout.flush()?;
                } else {
                    self.quitting = true;
                    self.undoing = false;
                    self.queue_status_text()?;
                    self.stdout.flush()?;
                }
            }
            // confirm quit or undo
            KeyCode::Char('y') => {
                if self.undoing {
                    self.selected = None;
                    self.promoting = None;
                    self.victor = None;
                    self.board.undo_last_move();
                    self.undoing = false;
                    self.queue_board()?;
                    self.queue_captured_pieces()?;
                    self.queue_status_text()?;
                    queue!(self.stdout, cursor::MoveTo(pos.0, pos.1))?;
                    self.stdout.flush()?;
                }
                if self.quitting {
                    return Ok(true);
                }
            }
            // stop undoing/quitting, or promote to knight
            KeyCode::Char('n') => {
                if self.undoing {
                    self.undoing = false;
                    self.queue_status_text()?;
                    self.stdout.flush()?;
                } else if self.quitting {
                    self.quitting = false;
                    self.queue_status_text()?;
                    self.stdout.flush()?;
                } else if let Some(promoting) = self.promoting {
                    self.board
                        .promote_pawn(promoting.0, promoting.1, PieceType::Knight);
                    if let Some(ref mut netplay) = self.netplay {
                        netplay.send(Command::Promote {
                            x: promoting.0,
                            y: promoting.1,
                            piece: PieceType::Knight,
                        })?;
                    }
                    self.promoting = None;
                    self.check_victor();
                    self.queue_board()?;
                    self.queue_status_text()?;
                    queue!(self.stdout, cursor::MoveTo(pos.0, pos.1))?;
                    self.stdout.flush()?;
                }
            }
            // deselect or stop undoing/quitting
            KeyCode::Esc => {
                if self.selected.is_some() {
                    self.selected = None;
                    self.queue_board()?;
                    queue!(self.stdout, cursor::MoveTo(pos.0, pos.1))?;
                    self.stdout.flush()?;
                }
                if self.quitting {
                    self.quitting = false;
                    self.queue_status_text()?;
                    self.stdout.flush()?;
                }
                if self.undoing {
                    self.undoing = false;
                    self.queue_status_text()?;
                    self.stdout.flush()?;
                }
            }
            // select or move piece
            KeyCode::Char(' ') => {
                if !can_move {
                    return Ok(false);
                }
                self.quitting = false;
                self.undoing = false;
                if pos.0 > MAX_X || pos.1 > MAX_Y {
                    return Ok(false);
                }
                #[allow(clippy::cast_possible_truncation)]
                let x = (pos.0 / SPACE_WIDTH) as u8;
                #[allow(clippy::cast_possible_truncation)]
                let y = 7 - (pos.1 / SPACE_HEIGHT) as u8;
                if let Some(s) = self.selected {
                    if s.0 == x && s.1 == y {
                        let space = self.board.space(x, y);
                        let colors = get_term_colors(space);
                        execute!(
                            self.stdout,
                            style::PrintStyledContent(space.draw().with(colors.0).on_black()),
                            cursor::MoveLeft(1)
                        )?;
                        self.selected = None;
                    } else if x < 8 && y < 8 && self.board.move_piece(s.0, s.1, x, y) {
                        if let Some(ref mut netplay) = self.netplay {
                            netplay.send(Command::Move {
                                x0: s.0,
                                y0: s.1,
                                x1: x,
                                y1: y,
                            })?;
                        }
                        self.selected = None;
                        self.promoting = {
                            let piece = self.board.space(x, y).piece().as_ref().unwrap();
                            if piece.piece_type() == PieceType::Pawn
                                && ((piece.color() == Color::White && y == 7)
                                    || (piece.color() == Color::Black && y == 0))
                            {
                                Some((x, y))
                            } else {
                                None
                            }
                        };

                        self.check_victor();
                        self.queue_board()?;
                        self.queue_captured_pieces()?;
                        queue!(
                            self.stdout,
                            cursor::MoveTo(
                                u16::from(x) * SPACE_WIDTH + (SPACE_WIDTH / 2),
                                (7 - u16::from(y)) * SPACE_HEIGHT + (SPACE_HEIGHT / 2)
                            )
                        )?;
                        self.queue_status_text()?;
                        self.stdout.flush()?;
                    }
                } else if x < 8 && y < 8 {
                    let space = self.board.space(x, y);
                    if let Some(piece_color) = space.piece_color() {
                        if piece_color == self.board.turn_color() {
                            self.selected = Some((x, y));
                            self.queue_space(x, y)?;
                            queue!(
                                self.stdout,
                                cursor::MoveUp(MIN_Y),
                                cursor::MoveLeft(MIN_X + 1)
                            )?;
                            self.stdout.flush()?;
                        }
                    }
                }
            }
            _ => {}
        }

        Ok(false)
    }

    fn check_victor(&mut self) {
        if self.board.is_in_checkmate(self.board.turn_color()) {
            self.victor = Some(match self.board.turn_color() {
                Color::Black => Color::White,
                Color::White => Color::Black,
            });
        }
    }

    fn queue_board(&mut self) -> Result<()> {
        for y in 0u8..8u8 {
            for x in 0u8..8u8 {
                self.queue_space(x, 7 - y)?;
            }
        }

        Ok(())
    }

    fn queue_space(&mut self, space_x: u8, space_y: u8) -> Result<()> {
        let space = self.board.space(space_x, space_y);
        let highlighted = if let Some(selected) = self.selected {
            selected.0 == space_x && selected.1 == space_y
        } else {
            false
        };
        let x = u16::from(space_x) * SPACE_WIDTH;
        let y = (7 - u16::from(space_y)) * SPACE_HEIGHT;
        let (fg_color, bg_color) = get_term_colors(space);
        queue!(
            self.stdout,
            cursor::MoveTo(x, y),
            style::SetBackgroundColor(bg_color),
            style::Print(' '),
            style::Print(' '),
            style::Print(' '),
            style::Print(' '),
            style::Print(' '),
            cursor::MoveTo(x, y + 1),
            style::Print(' '),
            style::Print(' '),
            if highlighted {
                style::PrintStyledContent(space.draw().black().on_green())
            } else if space.piece().is_some() {
                style::PrintStyledContent(space.draw().with(fg_color).on_black())
            } else {
                style::PrintStyledContent(space.draw().with(fg_color).on(bg_color))
            },
            style::SetBackgroundColor(bg_color),
            style::Print(' '),
            style::Print(' '),
            cursor::MoveTo(x, y + 2),
            style::Print(' '),
            style::Print(' '),
            style::Print(' '),
            style::Print(' '),
            style::Print(' '),
        )?;

        Ok(())
    }

    fn queue_status_text(&mut self) -> Result<()> {
        let pos = cursor::position()?;

        let (status, color) = if self.quitting {
            ("QUIT? (y/n)                ", TermColor::Magenta)
        } else if self.undoing {
            ("UNDO? (y/n)                ", TermColor::Magenta)
        } else if self.victor.is_some() {
            match self.victor.unwrap() {
                Color::White => ("WHITE WINS!                 ", TermColor::Magenta),
                Color::Black => ("BLACK WINS!                 ", TermColor::Magenta),
            }
        } else if self.promoting.is_some() {
            ("SELECT PROMOTION: (q/r/b/n)", TermColor::Magenta)
        } else {
            match self.board.turn_color() {
                Color::White => ("WHITE                      ", TermColor::Green),
                Color::Black => ("BLACK                      ", TermColor::Red),
            }
        };

        queue!(
            self.stdout,
            cursor::MoveTo(1, SPACE_HEIGHT * 8 + 1),
            style::PrintStyledContent(status.with(color).on_black()),
            cursor::MoveTo(pos.0, pos.1),
        )?;

        Ok(())
    }

    fn queue_captured_pieces(&mut self) -> Result<()> {
        let pos = cursor::position()?;

        let white = self.board.captured_by_white().clone();
        let black = self.board.captured_by_black().clone();

        let x_start = SPACE_WIDTH * 8 + SPACE_WIDTH / 2;

        queue!(
            self.stdout,
            cursor::MoveTo(x_start, 1),
            style::SetForegroundColor(TermColor::Green),
            style::SetBackgroundColor(TermColor::Black),
        )?;

        // blank rows, needed if a single piece was captured and then undone
        for _ in 0..=5 {
            queue!(
                self.stdout,
                style::Print("                "),
                cursor::MoveDown(1),
                cursor::MoveLeft(16)
            )?;
        }
        queue!(self.stdout, cursor::MoveTo(x_start, 1))?;

        self.queue_captured_row(&black, PieceType::Queen, false)?;
        self.queue_captured_row(&black, PieceType::Rook, false)?;
        self.queue_captured_row(&black, PieceType::Bishop, false)?;
        self.queue_captured_row(&black, PieceType::Knight, false)?;
        self.queue_captured_row(&black, PieceType::Pawn, false)?;

        let x_start = SPACE_WIDTH * 8 + SPACE_WIDTH / 2;
        let y_start = SPACE_HEIGHT * 8 - 1;

        queue!(
            self.stdout,
            cursor::MoveTo(x_start, y_start),
            style::SetForegroundColor(TermColor::Red),
        )?;

        // blank rows
        for _ in 0..=5 {
            queue!(
                self.stdout,
                style::Print("                "),
                cursor::MoveUp(1),
                cursor::MoveLeft(16)
            )?;
        }
        queue!(self.stdout, cursor::MoveTo(x_start, y_start))?;

        self.queue_captured_row(&white, PieceType::Pawn, true)?;
        self.queue_captured_row(&white, PieceType::Knight, true)?;
        self.queue_captured_row(&white, PieceType::Bishop, true)?;
        self.queue_captured_row(&white, PieceType::Rook, true)?;
        self.queue_captured_row(&white, PieceType::Queen, true)?;

        queue!(self.stdout, cursor::MoveTo(pos.0, pos.1), style::ResetColor)?;

        Ok(())
    }

    fn queue_captured_row(
        &mut self,
        pieces: &HashMap<PieceType, u8>,
        piece_type: PieceType,
        reverse: bool,
    ) -> Result<()> {
        let s = match piece_type {
            PieceType::Queen => "Q ",
            PieceType::Rook => "R ",
            PieceType::Bishop => "B ",
            PieceType::Knight => "N ",
            PieceType::Pawn => "P ",
            PieceType::King => "K ",
        };
        if pieces.contains_key(&piece_type) {
            let count = u16::from(*pieces.get(&piece_type).unwrap());
            queue!(self.stdout, style::Print(s.repeat(count as usize)),)?;
            if reverse {
                queue!(self.stdout, cursor::MoveUp(1))?;
            } else {
                queue!(self.stdout, cursor::MoveDown(1))?;
            }
            queue!(self.stdout, cursor::MoveLeft(count * 2))?;
        }
        Ok(())
    }
}

impl Default for Game {
    fn default() -> Self {
        Game::new()
    }
}

fn get_term_colors(space: &Space) -> (TermColor, TermColor) {
    let piece_color = match space.piece_color() {
        Some(Color::White) => TermColor::Green,
        Some(Color::Black) => TermColor::Red,
        None => TermColor::White,
    };
    let space_color = match space.color() {
        Color::White => TermColor::Grey,
        Color::Black => TermColor::Black,
    };
    (piece_color, space_color)
}

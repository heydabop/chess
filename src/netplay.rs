use std::io::{stdin, stdout, Read, Result, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};

use crate::piece::PieceType;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command {
    Ack,
    Move { x0: u8, y0: u8, x1: u8, y1: u8 },
    Promote { x: u8, y: u8, piece: PieceType },
}

impl Command {
    fn to_packet(self) -> Vec<u8> {
        match self {
            Self::Ack => vec![1],
            Self::Move { x0, y0, x1, y1 } => vec![2, x0, y0, x1, y1],
            Self::Promote { x, y, piece } => vec![3, x, y, piece as u8],
        }
    }

    fn packet_length(self) -> u8 {
        match self {
            Self::Ack => 1,
            Self::Move { .. } => 5,
            Self::Promote { .. } => 4,
        }
    }
}

pub struct NetPlay<S: Read + Write> {
    is_host: bool,
    stream: S,
}

impl<S: Read + Write> NetPlay<S> {
    pub fn new(is_host: bool, stream: S) -> Self {
        Self { is_host, stream }
    }

    pub fn is_host(&self) -> bool {
        self.is_host
    }

    fn send_command(&mut self, command: Command) -> Result<()> {
        let packet = command.to_packet();
        self.stream.write_all(&packet)?;
        self.stream.flush()?;
        Ok(())
    }

    fn recv_command(&mut self) -> Result<Command> {
        let mut type_buf = [0];
        self.stream.read_exact(&mut type_buf)?;
        let packet_type = type_buf[0];
        if packet_type == 1 {
            return Ok(Command::Ack);
        }
        let remaining_length = match packet_type {
            2 => 4,
            3 => 3,
            _ => panic!("unrecognized packet type {packet_type}"),
        };
        let mut buf = vec![0; remaining_length];
        self.stream.read_exact(&mut buf)?;
        let command = match packet_type {
            2 => Command::Move {
                x0: buf[0],
                y0: buf[1],
                x1: buf[2],
                y1: buf[3],
            },
            3 => Command::Promote {
                x: buf[0],
                y: buf[1],
                piece: buf[2].into(),
            },
            _ => unreachable!(),
        };

        Ok(command)
    }

    pub fn send(&mut self, command: Command) -> Result<()> {
        self.send_command(command)?;
        let response = self.recv_command()?;
        if let Command::Ack = response {
            return Ok(());
        }
        panic!("expected ack, got {response:?}");
    }

    pub fn recv(&mut self) -> Result<Command> {
        let command = self.recv_command()?;
        self.send_command(Command::Ack)?;
        Ok(command)
    }
}

impl NetPlay<TcpStream> {
    pub fn init_tcp_from_stdin() -> Result<Option<Self>> {
        loop {
            let buffer = prompt("Netplay? (y/n): ")?;
            let c = buffer.chars().next();
            if c == Some('y') {
                break;
            }
            if c == Some('n') {
                return Ok(None);
            }
        }
        let host = loop {
            let buffer = prompt("Host? (y/n): ")?;
            let c = buffer.chars().next();
            if c == Some('y') {
                break true;
            }
            if c == Some('n') {
                break false;
            }
        };
        if host {
            let listener = TcpListener::bind("0.0.0.0:0")?;
            let port = listener.local_addr()?.port();
            println!("Awaiting connection on port {port}");
            let (stream, _) = listener.accept()?;
            return Ok(Some(Self::new(true, stream)));
        }
        let address = loop {
            let buffer = prompt("Destination IP Address: ")?;
            match buffer.trim().parse::<Ipv4Addr>() {
                Ok(a) => break a,
                Err(_) => {
                    println!("Invalid IP address");
                }
            }
        };
        let port = loop {
            let buffer = prompt("Destination Port: ")?;
            match buffer.trim().parse::<u16>() {
                Ok(p) => break p,
                Err(_) => {
                    println!("Invalid port");
                }
            }
        };
        let socket = SocketAddrV4::new(address, port);
        let stream = TcpStream::connect(socket)?;
        Ok(Some(Self::new(false, stream)))
    }
}

fn prompt(s: &'static str) -> Result<String> {
    let mut buffer = String::new();
    stdout().write_all(s.as_bytes())?;
    stdout().flush()?;
    stdin().read_line(&mut buffer)?;
    stdout().write_all(b"\n")?;
    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

    #[test]
    fn ack() -> Result<()> {
        let mut netplay = NetPlay::new(false, VecDeque::new());
        netplay.send_command(Command::Ack)?;
        assert_eq!(netplay.recv_command()?, Command::Ack);
        Ok(())
    }

    #[test]
    fn r#move() -> Result<()> {
        let mut netplay = NetPlay::new(false, VecDeque::new());
        let move_command = Command::Move {
            x0: 0,
            y0: 1,
            x1: 2,
            y1: 3,
        };
        netplay.send_command(move_command)?;
        assert_eq!(netplay.recv_command()?, move_command);
        Ok(())
    }

    #[test]
    fn promote() -> Result<()> {
        let mut netplay = NetPlay::new(false, VecDeque::new());
        let promote = Command::Promote {
            x: 6,
            y: 7,
            piece: PieceType::Queen,
        };
        netplay.send_command(promote)?;
        assert_eq!(netplay.recv_command()?, promote);
        Ok(())
    }

    #[test]
    fn multiple_commands() -> Result<()> {
        let mut netplay = NetPlay::new(false, VecDeque::new());
        let move1 = Command::Move {
            x0: 0,
            y0: 1,
            x1: 2,
            y1: 3,
        };
        let promote = Command::Promote {
            x: 6,
            y: 7,
            piece: PieceType::Queen,
        };
        let move2 = Command::Move {
            x0: 4,
            y0: 5,
            x1: 6,
            y1: 7,
        };
        netplay.send_command(move1)?;
        netplay.send_command(promote)?;
        netplay.send_command(move2)?;
        assert_eq!(netplay.recv_command()?, move1);
        assert_eq!(netplay.recv_command()?, promote);
        assert_eq!(netplay.recv_command()?, move2);
        Ok(())
    }
}

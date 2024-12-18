extern crate termion;

use std::io::{stdin, stdout, Stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::terminal_size;

pub struct Editor {

}

impl Editor {
    pub fn default() -> Self {
        Editor{}
    }

    fn print_empty_file(stdout: &mut RawTerminal<Stdout>) {
        write!(
            stdout,
            "{}",
            termion::cursor::Goto(1, 1)
        ).unwrap();
        for _ in 0..terminal_size().unwrap().1 {
            print!("~\n");
        }
        stdout.flush().unwrap();
    } 

    pub fn run(&self) {
        // Put the terminal into raw mode
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();

        // Clear the screen
        write!(
            stdout, 
            "{}{}", 
            termion::clear::All,
            termion::cursor::Goto(1, 1)
        ).unwrap();
        stdout.flush().unwrap();

        Editor::print_empty_file(&mut stdout);

        for key in stdin.keys() {
            match key.as_ref() {
                Ok(k) => {
                    match k {
                        Key::Char(c) => print!("{c}"),
                        Key::Ctrl('c') => break,
                        Key::Up => print!("{}", termion::cursor::Up(1)),
                        Key::Down => print!("{}", termion::cursor::Down(1)),
                        Key::Left => print!("{}", termion::cursor::Left(1)),
                        Key::Right => print!("{}", termion::cursor::Right(1)),
                        Key::Backspace => print!("{} {}", termion::cursor::Left(1), termion::cursor::Left(1)),
                        _ => {
                            // Do nothing, for now. . .
                        }
                    }
                }
                Err(err) => {
                    println!("ERROR: {err}");
                }
            }
            stdout.flush().unwrap();
        }
    }
}
extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {

}

impl Editor {
    pub fn default() -> Self {
        Editor{}
    }

    pub fn run(&self) {
        // Put the terminal into raw mode
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();

        // Clear the screen
        write!(
            stdout, "{}{}", 
            termion::clear::All,
            termion::cursor::Goto(1, 1)
        ).unwrap();
        stdout.flush().unwrap();

        for key in stdin.keys() {
            match key.as_ref() {
                Ok(k) => {
                    match k {
                        Key::Char(c) => print!("{c}"),
                        Key::Ctrl('c') => break,
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
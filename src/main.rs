extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
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

    for k in stdin.keys() {
        match k.as_ref().unwrap() {
            Key::Char(c) => print!("{}", c),
            Key::Ctrl('c') => break,
            _ => {
                // Do nothing, for now. . .
            }
        }
        stdout.flush().unwrap();
    }
}

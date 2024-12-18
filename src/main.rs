use termion::raw::IntoRawMode;
use termion::clear;
use std::io::{stdin, stdout, Read, Write};

fn main() {
    // Put the terminal into raw mode
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();
    stdout.activate_raw_mode().unwrap();

    // Clear the screen
    print!("{}", clear::All);
    stdout.flush().unwrap();

    for b in stdin.bytes() {
        let b = b.unwrap();
        let c = b as char;

        // Print the recieved character to the console
        print!("{}", c);
        stdout.flush().unwrap();

        // Quit the program when Ctrl-C is pressed
        if b == 3 {
            stdout.suspend_raw_mode().unwrap();
            break;
        }
    }
}

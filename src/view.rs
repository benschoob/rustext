/*
Text rendering and event handling.
Renders the buffer onscreen and modifies it with text events
*/

use crate::buffer::Buffer;

pub enum TextEvent {
    Char(char),
    Up,
    Down,
    Left,
    Right,
    Backspace,
}

pub struct View {
    buffer: Buffer,
}

impl View {
    pub fn new() -> Self {
        View {
            buffer: Buffer::new(),
        }
    }

    pub fn handle_text_event(&self, event: TextEvent) {

    }

    pub fn render(&self) {

    }
}
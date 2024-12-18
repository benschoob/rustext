extern crate termion;

mod buffer;
mod view;
mod editor;

use editor::Editor;
use view::View;
use buffer::Buffer;

fn main() {
    // Initialize editor structs
    let view = View::new();
    let buffer = Buffer::new();
    let editor = Editor::default();

    editor.run(view, buffer);
}

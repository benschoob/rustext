/*
The character buffer. Holds the data of the text file we are editing, reflecting any edits that have been made
*/
pub struct Buffer {
    buf: Vec<u8>
}

impl Buffer {
    pub fn new() -> Self {
        Buffer {
            buf: Vec::new(),
        }
    }
}
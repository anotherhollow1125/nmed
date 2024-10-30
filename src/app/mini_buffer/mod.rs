use ratatui::crossterm::event::KeyCode;

pub mod ui;

/// Command Widget Structure
pub struct MiniBuffer {
    pub prompt_key: KeyCode,
    buffer: Option<MiniBufferBuffer>,
}

pub struct MiniBufferBuffer {
    s: String,
}

impl MiniBufferBuffer {
    pub fn new(prompt_key: KeyCode) -> Self {
        MiniBufferBuffer { s: String::new() }
    }

    pub fn write(&mut self, c: char) {
        self.s.push(c);
    }

    pub fn rend(&self) -> &str {
        &self.s
    }
}

impl MiniBuffer {
    pub fn new(prompt_key: KeyCode) -> Self {
        MiniBuffer {
            prompt_key,
            buffer: None,
        }
    }

    pub fn area_height(&self) -> u16 {
        1
    }

    pub fn open(&mut self) {
        self.buffer = Some(MiniBufferBuffer::new(self.prompt_key));
    }

    pub fn close(&mut self) {
        self.buffer = None;
    }

    pub fn get_buf(&self) -> Option<&MiniBufferBuffer> {
        self.buffer.as_ref()
    }

    pub fn mut_buf(&mut self) -> Option<&mut MiniBufferBuffer> {
        self.buffer.as_mut()
    }
}

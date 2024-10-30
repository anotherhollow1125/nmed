use ratatui::crossterm::event::KeyCode;

pub mod ui;

pub struct Assist {
    pub assist_key: KeyCode,
    buffer: Option<AssistBuffer>,
    key_map: KeyMap,
}

pub struct AssistBuffer {
    s: String,
}

impl AssistBuffer {
    pub fn new() -> Self {
        AssistBuffer { s: String::new() }
    }

    pub fn write(&mut self, c: char) {
        self.s.push(c);
    }

    pub fn rend(&self) -> &str {
        &self.s
    }
}

impl Assist {
    pub fn new(assist_key: KeyCode) -> Self {
        Assist {
            assist_key,
            buffer: None,
            key_map: KeyMap,
        }
    }

    pub fn area_height(&self) -> u16 {
        1
    }

    pub fn open(&mut self) {
        self.buffer = Some(AssistBuffer::new());
    }

    pub fn close(&mut self) {
        self.buffer = None;
    }

    pub fn get_buf(&self) -> Option<&AssistBuffer> {
        self.buffer.as_ref()
    }

    pub fn mut_buf(&mut self) -> Option<&mut AssistBuffer> {
        self.buffer.as_mut()
    }
}

struct KeyMap;

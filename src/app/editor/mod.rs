pub mod ui;

pub struct Editor {
    pub current_buffer: usize,
    pub buffers: Vec<Buffer>, // 将来的に複数バッファを扱う
}

pub struct Buffer {
    file_path: Option<String>,
    current_line: usize,
    current_column: usize,
    lines: Vec<String>,
}

impl Buffer {
    pub fn write(&mut self, c: char) {
        self.lines[self.current_line].push(c);
    }

    // 後で対象行・列のみを返すように変更する
    pub fn rend(&self) -> &[String] {
        &self.lines
    }
}

impl Editor {
    pub fn new() -> Self {
        let buffer = Buffer {
            file_path: None,
            current_line: 0,
            current_column: 0,
            lines: vec![String::new()],
        };

        Editor {
            current_buffer: 0,
            buffers: vec![buffer],
        }
    }

    pub fn mut_current_buf(&mut self) -> &mut Buffer {
        &mut self.buffers[self.current_buffer]
    }

    pub fn get_current_buf(&self) -> &Buffer {
        &self.buffers[self.current_buffer]
    }
}

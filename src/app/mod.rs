pub mod assist;
pub mod cli_args;
pub mod editor;
pub mod mini_buffer;
pub mod ui;

use assist::Assist;
pub use cli_args::Args;
use color_eyre::eyre::eyre;
use color_eyre::Result;
use editor::Editor;
use mini_buffer::MiniBuffer;

pub struct App {
    pub editor: Editor,
    pub assist: Assist,
    pub mini_buffer: MiniBuffer,
    col: usize,
    row: usize,
}

impl App {
    pub fn new(args: Args) -> Result<Self> {
        let Args {
            prompt_key,
            assist_key,
        } = args;

        if prompt_key == assist_key {
            return Err(eyre!(
                "prompt_key and assist_key must be different\nnow: prompt_key = {} assist_key = {}",
                prompt_key,
                assist_key
            ));
        }

        Ok(App {
            editor: Editor::new(),
            assist: Assist::new(assist_key),
            mini_buffer: MiniBuffer::new(prompt_key),
            col: 0,
            row: 0,
        })
    }
}

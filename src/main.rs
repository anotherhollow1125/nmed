use clap::Parser;
use color_eyre::eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent},
    prelude::Backend,
    Terminal,
};

mod app;

use app::App;
use app::Args;
use app::{assist::AssistBuffer, mini_buffer::MiniBufferBuffer, ui::ui};

fn main() -> Result<()> {
    let args = Args::parse();

    let mut terminal = ratatui::try_init()?;
    let mut app = app::App::new(args)?;

    run_app(&mut terminal, &mut app)?;

    ratatui::try_restore()?;
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        let Event::Key(KeyEvent {
            kind: event::KeyEventKind::Press,
            code,
            ..
        }) = event::read()?
        else {
            continue;
        };

        let signal = match (app.assist.mut_buf(), app.mini_buffer.mut_buf()) {
            (Some(buf), _) => assist_input(buf, code),
            (_, Some(buf)) => mini_buffer_input(buf, code),
            _ => editor_input(app, code),
        }?;

        match signal {
            Signal::CloseAssist => app.assist.close(),
            Signal::CloseMiniBuffer => app.mini_buffer.close(),
            Signal::Exit => break,
            Signal::Continue => {}
        }
    }

    Ok(())
}

enum Signal {
    CloseAssist,
    CloseMiniBuffer,
    Exit,
    Continue,
}

fn assist_input(buf: &mut AssistBuffer, code: KeyCode) -> Result<Signal> {
    if let KeyCode::Char(c) = code {
        buf.write(c);
    }

    Ok(Signal::Continue)
}

fn mini_buffer_input(buf: &mut MiniBufferBuffer, code: KeyCode) -> Result<Signal> {
    match code {
        KeyCode::Enter => {
            if buf.rend() == "exit" || buf.rend() == "quit" || buf.rend() == "q" {
                Ok(Signal::Exit)
            } else {
                Ok(Signal::Continue)
            }
        }
        KeyCode::Esc => Ok(Signal::CloseMiniBuffer),
        KeyCode::Char(c) => {
            buf.write(c);
            Ok(Signal::Continue)
        }
        _ => Ok(Signal::Continue),
    }
}

fn editor_input(app: &mut App, code: KeyCode) -> Result<Signal> {
    match code {
        k if k == app.assist.assist_key => app.assist.open(),
        k if k == app.mini_buffer.prompt_key => app.mini_buffer.open(),
        KeyCode::Char(c) => app.editor.mut_current_buf().write(c),
        _ => {}
    }

    Ok(Signal::Continue)
}

use ratatui::{
    style::{Color, Style},
    widgets::Paragraph,
};

use crate::app::mini_buffer::MiniBuffer;

pub fn mini_buffer_widget(mini_buffer: &MiniBuffer) -> Paragraph<'_> {
    match mini_buffer.get_buf().map(|buf| buf.rend()) {
        Some(content) => Paragraph::new(format!("{}{}", mini_buffer.prompt_key, content)),
        None => Paragraph::new(format!(
            "press '{}' to open command prompt",
            mini_buffer.prompt_key
        ))
        .style(Style::default().fg(Color::DarkGray)),
    }
}

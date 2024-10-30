use ratatui::widgets::{Paragraph, Wrap};

use crate::app::App;

pub fn editor_widget(app: &App) -> Paragraph<'_> {
    let content = app.editor.get_current_buf().rend();
    Paragraph::new(content[0].as_str()).wrap(Wrap { trim: false })
}

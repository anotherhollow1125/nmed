use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};

use crate::app::assist::ui::popup_assist_widget;
use crate::app::editor::ui::editor_widget;
use crate::app::mini_buffer::ui::mini_buffer_widget;
use crate::app::App;

pub fn ui(frame: &mut Frame, app: &App) {
    let AppArea {
        editor_area,
        mode_line_area,
        mini_buffer_area,
    } = define_area(frame, app);

    frame.render_widget(editor_widget(app), editor_area);
    frame.render_widget(mode_line_widget(), mode_line_area);
    frame.render_widget(mini_buffer_widget(&app.mini_buffer), mini_buffer_area);
}

struct AppArea {
    editor_area: Rect,
    // assist_area is popup widget
    mode_line_area: Rect,
    mini_buffer_area: Rect,
}

fn define_area(frame: &Frame, app: &App) -> AppArea {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),                                // editor
            Constraint::Length(1),                             // mode line
            Constraint::Length(app.mini_buffer.area_height()), // mini buffer area
        ])
        .split(frame.area());

    AppArea {
        editor_area: chunks[0],
        mode_line_area: chunks[1],
        mini_buffer_area: chunks[2],
    }
}

fn mode_line_widget() -> Paragraph<'static> {
    Paragraph::new(" mode line").style(Style::default().bg(Color::White).fg(Color::Black))
}

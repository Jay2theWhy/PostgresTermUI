use crate::app::{App, InputMode};
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    widgets::{Block, Paragraph, Borders},
    style::{Color, Style},
    Frame,
};

pub fn render<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {

    let (title, color) = match app.input_mode {
        InputMode::Normal => (" Current mode: Normal ", Color::Blue),
        InputMode::Editing => (" Current mode: Edit ", Color::Magenta),
    };

    let default_style = Style::default().fg(color);

    let block = Block::default()
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .style(default_style);

    let input = Paragraph::new(title)
        .alignment(Alignment::Center)
        .block(block);

    f.render_widget(input, area);
}


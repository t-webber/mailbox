//! Module to define reusable TUI components for the app

use ratatui::layout::Alignment;
use ratatui::style::{Style, Stylize as _};
use ratatui::text::Span;
use ratatui::widgets::{Block, BorderType};

pub fn title<'title>(title: &'title str) -> Span<'title> {
    Span::styled(title, Style::default().bold())
}

pub fn new_simple_box(title: &str) -> Block<'_> {
    Block::bordered()
        .title(title)
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
}

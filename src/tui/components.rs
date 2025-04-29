//! Module to define reusable TUI components for the app

use ratatui::style::{Style, Stylize as _};
use ratatui::text::Span;

pub fn title<'title>(title: &'title str) -> Span<'title> {
    Span::styled(title, Style::default().bold())
}

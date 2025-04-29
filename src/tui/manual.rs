//! Defines the manual page to render on the screen
//!
//! This manual explains usage and the different keybindings for the app.

use ratatui::Frame;
use ratatui::style::{Style, Stylize as _};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Paragraph, Wrap};

/// Bold styling
fn bold(title: &str) -> Span<'_> {
    Span::styled(title, Style::default().bold())
}

/// Displays the manual page to the current frame
pub fn manual_page(frame: &mut Frame<'_>) {
    let lines = vec![
        Line::from(bold("mailbox-tui")),
        Line::from(""),
        Line::from("A TUI app to read, write and manage emails."),
        Line::from(""),
        Line::from(bold("Presentation")),
        Line::from(""),
        Line::from("The app contains multiple modes"),
        Line::from(""),
        Line::from("> Press 'q' to exit the application."),
        Line::from(""),
        Line::from(bold("Modes")),
        Line::from(""),
        Line::from("- Manual mode (press 'm' to enable)"),
        Line::from("- Writer mode (press 'w' to enable)"),
        Line::from("- Reader mode (press 'r' to enable)"),
        Line::from(""),
        Line::from(bold("Manual mode")),
        Line::from("This is manual mode. To switch de manual mode, press 'm'."),
        Line::from(""),
        Line::from(bold("Read mode")),
        Line::from(""),
        Line::from(
            "Mode to display emails from an inbox. Press 'r' to switch to this mode.",
        ),
        Line::from(""),
        Line::from("Keybindings:"),
        Line::from("- 'k': select previous email"),
        Line::from("- 'j': select next email"),
        Line::from("- 'h': close email reader"),
        Line::from("- 'm': open email reader"),
        Line::from(""),
        Line::from(bold("Write mode")),
        Line::from(""),
        Line::from("Mode to write emails. Press 'w' to switch to this mode."),
    ];

    let help = Paragraph::new(Text::from(lines)).wrap(Wrap { trim: false });
    frame.render_widget(help, frame.area());
}

//! Runs and manages the TUI and its interactions.

use core::any::Any;
use std::io::stdout;
use std::process::exit;
use std::{io, thread};

use ratatui::Frame;
use ratatui::crossterm::QueueableCommand as _;
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, read};
use ratatui::crossterm::terminal::{Clear, ClearType, disable_raw_mode};
use ratatui::text::Text;

use crate::errors::Result;

/// Main runner for the TUI
///
/// Handles key events and frame renders
pub fn run() -> Result {
    let mut terminal = ratatui::init();

    thread::spawn(key_events);

    loop {
        terminal.draw(draw).map_err(Error::Drawing)?;
    }
}

/// Handles key events
#[expect(clippy::exit, reason = "kill all processes and exit")]
fn key_events() -> Result {
    loop {
        match read().map_err(Error::IoKeyboard)? {
            Event::Key(KeyEvent { code: KeyCode::Char('q'), .. }) => {
                stdout()
                    .queue(Clear(ClearType::All))
                    .map_err(Error::ClearTerminal)?;
                disable_raw_mode().map_err(Error::DisablingRawMode)?;
                exit(0);
            }
            Event::Key(_)
            | Event::FocusGained
            | Event::FocusLost
            | Event::Mouse(_)
            | Event::Paste(_)
            | Event::Resize(..) => (),
        }
    }
}

/// Draws 'Hello world' onto the frame
fn draw(frame: &mut Frame<'_>) {
    let text = Text::raw("Hello World");
    frame.render_widget(text, frame.area());
}

/// Errors than occur because of the TUI rendering
#[derive(Debug)]
pub enum Error {
    /// Failed to clear the terminal
    ClearTerminal(io::Error),
    /// Failed to disable raw terminal mode.
    ///
    /// See [`disable_raw_mode`] for more information.
    DisablingRawMode(io::Error),
    /// Error occurred while drawing a frame.
    Drawing(io::Error),
    /// Error occurred while reading the keyboard presses.
    IoKeyboard(io::Error),
    /// Error occurred while spawning keyboard listener thread.
    UnknownKeyboard(Box<dyn Any + Send>),
}

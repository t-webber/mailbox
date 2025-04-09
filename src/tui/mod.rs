//! Runs and manages the TUI and its interactions.

use core::any::Any;
use std::io;

use ratatui::crossterm::event::{read, Event, KeyCode, KeyEvent};
use ratatui::text::Text;
use ratatui::Frame;

use crate::credentials::Credentials;
use crate::errors::Result;
use crate::fetch::connection::ImapSession;
use crate::fetch::parser::Email;

/// Follows the state of the TUI application.
#[derive(Default)]
pub struct Tui {
    /// Indicates whether the app is running
    running: bool,
}

impl Tui {
    /// Creates a new [`Tui`]
    pub const fn new() -> Self {
        Self { running: false }
    }

    /// Runs the [`Tui`]
    ///
    /// Handles key events and frame renders
    pub fn run(&mut self) -> Result {
        let credentials = Credentials::load()?;
        let mut session = ImapSession::with_credentials(&credentials)?.select_mailbox("INBOX")?;
        let email_uids = session.get_uids()?;
        let first_email_bodies = email_uids
            .into_iter()
            .take(20)
            .map(|uid| Ok((uid, session.get_mail_from_uid(uid)?)))
            .collect::<Result<Vec<_>>>()?;
        let first_emails = first_email_bodies
            .iter()
            .map(|(uid, body)| Ok(Email::try_from((*uid, body.as_bytes()))?))
            .collect::<Result<Vec<_>>>()?;

        let mut terminal = ratatui::init();
        self.running = true;
        while self.running {
            terminal
                .draw(|frame| draw_emails(frame, &first_emails))
                .map_err(Error::Drawing)?;
            self.key_events()?;
        }
        ratatui::restore();
        // disable_raw_mode().map_err(Error::DisablingRawMode)?;
        Ok(())
    }

    /// Handles key events
    fn key_events(&mut self) -> Result {
        match read().map_err(Error::IoKeyboard)? {
            Event::Key(KeyEvent { code: KeyCode::Char('q'), .. }) => {
                self.running = false;
            }
            Event::Key(_)
            | Event::FocusGained
            | Event::FocusLost
            | Event::Mouse(_)
            | Event::Paste(_)
            | Event::Resize(..) => (),
        }
        Ok(())
    }
}

/// Draws 'Hello world' onto the frame
fn draw_emails(frame: &mut Frame<'_>, emails: &[Email]) {
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

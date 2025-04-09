//! Runs and manages the TUI and its interactions.

use core::any::Any;
use std::io;

use mail_parser::HeaderName;
use ratatui::crossterm::event::{read, Event, KeyCode, KeyEvent};
use ratatui::widgets::{Block, List};
use ratatui::Frame;

use crate::credentials::Credentials;
use crate::errors::Result;
use crate::fetch::connection::ImapSession;
use crate::fetch::parser::{self, Email};

/// Follows the state of the TUI application.
#[derive(Default)]
pub struct Tui {
    /// Emails that were fetched from the server
    emails: Vec<Email>,
    /// Email uids that exist in the INBOX
    uids: Vec<u32>,
    /// Indicates whether the app is running
    running: bool,
}

impl Tui {
    /// Creates a new [`Tui`]
    pub fn new() -> Result<Self> {
        let credentials = Credentials::load()?;
        let mut session = ImapSession::with_credentials(&credentials)?.select_mailbox("INBOX")?;
        let uids = session.get_uids()?;
        let first_email_bodies = uids
            .iter()
            .take(20)
            .map(|uid| Ok((uid, session.get_mail_from_uid(*uid)?)))
            .collect::<Result<Vec<_>>>()?;
        let first_emails = first_email_bodies
            .iter()
            .map(|(uid, body)| Ok(Email::try_from((**uid, body.as_bytes()))?))
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { running: false, uids, emails: first_emails })
    }

    /// Runs the [`Tui`]
    ///
    /// Handles key events and frame renders
    #[expect(
        clippy::unwrap_in_result,
        clippy::unwrap_used,
        reason = "inside closure"
    )]
    pub fn run(&mut self) -> Result {
        let mut terminal = ratatui::init();
        self.running = true;
        while self.running {
            terminal
                .draw(|frame| draw_emails(frame, &self.emails).unwrap())
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
fn draw_emails(frame: &mut Frame<'_>, emails: &[Email]) -> Result {
    let email_subjects = emails
        .iter()
        .map(|email| {
            Ok(email
                .get_header(&HeaderName::Subject)?
                .as_text()
                .ok_or(parser::Error::InvalidHeaderType)?
                .to_owned())
        })
        .collect::<Result<Vec<_>>>()?;
    let emails_list = List::new(email_subjects);
    let block = emails_list.block(Block::default());
    frame.render_widget(block, frame.area());
    Ok(())
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

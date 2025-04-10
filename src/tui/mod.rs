//! Runs and manages the TUI and its interactions.

use core::any::Any;
use std::io;

use mail_parser::HeaderName;
use ratatui::crossterm::event::{read, Event, KeyCode, KeyEvent};
use ratatui::layout::Alignment;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, BorderType, List, ListItem, Padding};
use ratatui::Frame;

use crate::credentials::Credentials;
use crate::errors::Result;
use crate::fetch::connection::ImapSession;
use crate::fetch::parser::{self, Email};

/// Follows the state of the TUI application.
#[derive(Default)]
pub struct Tui {
    /// Id of the email that is hovered
    ///
    /// The id is computed from the most recent recent email (i.e., the latest
    /// email will be associated to an id of 0. When the list of emails is
    /// refetched from the server, this id must be synchronised to be
    /// coherent with the new email list.
    current_id: usize,
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

        Ok(Self { running: false, uids, emails: first_emails, current_id: 0 })
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
                .draw(|frame| self.draw_emails(frame).unwrap())
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
            Event::Key(KeyEvent { code: KeyCode::Char('j'), .. }) => {
                let incremented = self.current_id.saturating_add(1);
                if incremented < self.emails.len() {
                    self.current_id = incremented;
                }
            }
            Event::Key(KeyEvent { code: KeyCode::Char('k'), .. }) =>
                self.current_id = self.current_id.saturating_sub(1),
            Event::Key(_)
            | Event::FocusGained
            | Event::FocusLost
            | Event::Mouse(_)
            | Event::Paste(_)
            | Event::Resize(..) => (),
        }
        Ok(())
    }

    /// Draws 'Hello world' onto the frame
    fn draw_emails(&self, frame: &mut Frame<'_>) -> Result {
        let email_subjects = self
            .emails
            .iter()
            .enumerate()
            .map(|(id, email)| {
                let subject = email
                    .get_header(&HeaderName::Subject)?
                    .as_text()
                    .ok_or(parser::Error::InvalidHeaderType)?
                    .to_owned();
                let date = email
                    .get_header(&HeaderName::Date)?
                    .as_datetime()
                    .ok_or(parser::Error::InvalidHeaderType)?
                    .to_rfc3339();
                let raw_text = Text::from(vec![Line::from(subject), Line::from(date)]);
                let styled_text = if self.current_id == id {
                    raw_text.style(Style::new().bg(Color::Green))
                } else {
                    raw_text
                };
                Ok(ListItem::from(styled_text))
            })
            .collect::<Result<Vec<_>>>()?;

        let subject_list_container = Block::bordered()
            .title("Recent emails")
            .border_type(BorderType::Rounded)
            .title_alignment(Alignment::Center);
        let email_subject_list = List::new(email_subjects);

        let widget = email_subject_list.block(subject_list_container);

        frame.render_widget(widget, frame.area());

        Ok(())
    }
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

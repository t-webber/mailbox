//! Runs and manages the TUI and its interactions.

use core::any::Any;
use std::io;
use std::sync::Arc;

use mail_parser::HeaderName;
use ratatui::Frame;
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, read};
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, BorderType, List, ListItem};

use crate::credentials::Credentials;
use crate::errors::Result;
use crate::fetch;
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
    /// Id of the opened email
    ///
    /// This is the same id than `current_id`, so the same rules apply.
    open_email_id: Option<usize>,
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

        Ok(Self { running: false, uids, open_email_id: None, emails: first_emails, current_id: 0 })
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
            Event::Key(KeyEvent { code: KeyCode::Char('k'), .. }) => {
                self.current_id = self.current_id.saturating_sub(1);
            }
            Event::Key(KeyEvent { code: KeyCode::Char('l'), .. }) => {
                self.open_email_id = Some(self.current_id);
            }
            Event::Key(KeyEvent { code: KeyCode::Char('h'), .. }) => {
                self.open_email_id = None;
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

    /// Draws 'Hello world' onto the frame
    #[expect(
        clippy::missing_asserts_for_indexing,
        clippy::indexing_slicing,
        reason = "manual check"
    )]
    fn draw_emails(&self, frame: &mut Frame<'_>) -> Result {
        if let Some(open_email_id) = self.open_email_id {
            let layout =
                Layout::new(Direction::Horizontal, [Constraint::Fill(10), Constraint::Fill(10)])
                    .split(Rect::new(0, 0, frame.area().width, frame.area().height));

            if layout.len() != 2 {
                return Err(Error::LayoutFailure.into());
            }

            let email = &self.emails[open_email_id];
            frame.render_widget(self.get_email_explorer_widget()?, layout[0]);
            frame.render_widget(Self::get_email_viewer_widget(email)?, layout[1]);
        } else {
            frame.render_widget(self.get_email_explorer_widget()?, frame.area());
        }
        Ok(())
    }

    /// Creates the widget representing the email viewer
    ///
    /// This is the panel displaying the content of the selected email.
    fn get_email_viewer_widget(email: &Email) -> Result<List<'_>> {
        let subject_str = email.as_headers().get(&HeaderName::Subject).map_or_else(
            || Ok("No subject".to_owned()),
            |value| {
                value
                    .as_text()
                    .map(ToOwned::to_owned)
                    .ok_or(fetch::parser::Error::InvalidHeaderType)
            },
        )?;
        let subject_txt = Text::from(subject_str);

        let date_str = email.as_headers().get(&HeaderName::Date).map_or_else(
            || Ok("No date".to_owned()),
            |value| {
                value
                    .as_datetime()
                    .ok_or(fetch::parser::Error::InvalidHeaderType)
                    .map(mail_parser::DateTime::to_rfc3339)
            },
        )?;
        let date_txt = Text::from(date_str);

        let from_str = email.as_headers().get(&HeaderName::From).map_or_else(
            || Ok("No from".to_owned()),
            |value| {
                value
                    .as_address()
                    .ok_or(fetch::parser::Error::InvalidHeaderType)
                    .map(|address| format!("{address:?}"))
            },
        )?;
        let from_txt = Text::from(from_str);

        let email_view = List::new([subject_txt, date_txt, from_txt]);

        Ok(email_view)
    }

    /// Creates the widget representing the email explorer
    ///
    /// This is left panel of the editor, giving the list of received emails and enabling the user to select an email to display.
    fn get_email_explorer_widget(&self) -> Result<List<'_>> {
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

        let email_explorer = List::new(email_subjects).block(new_simple_box("Recent emails"));

        Ok(email_explorer)
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
    /// Failed to create the layout
    LayoutFailure,
    /// Error occurred while spawning keyboard listener thread.
    UnknownKeyboard(Box<dyn Any + Send>),
}

fn new_simple_box(title: &str) -> Block<'_> {
    Block::bordered()
        .title(title)
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
}

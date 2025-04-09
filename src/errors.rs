//! Handles errors, with a custom [`Result`] and [`Error`] type

use core::result;

use crate::{credentials, fetch, tui};

/// Errors that may occur while running the app.
#[derive(Debug)]
pub enum Error {
    /// `dotenv` failed to read the `.env` file.
    Credentials(credentials::Error),
    /// Failure occurred while interaction with the IMAP protocol.
    ImapConnection(fetch::connection::Error),
    /// Failure occurred while parsing the email body.
    Parsing(fetch::parser::Error),
    /// Failure occurred after TUI
    Tui(tui::Error),
}

impl From<credentials::Error> for Error {
    fn from(error: credentials::Error) -> Self {
        Self::Credentials(error)
    }
}

impl From<fetch::connection::Error> for Error {
    fn from(error: fetch::connection::Error) -> Self {
        Self::ImapConnection(error)
    }
}

impl From<fetch::parser::Error> for Error {
    fn from(error: fetch::parser::Error) -> Self {
        Self::Parsing(error)
    }
}

impl From<tui::Error> for Error {
    fn from(error: tui::Error) -> Self {
        Self::Tui(error)
    }
}

/// Overloaded result for the [`mailbox`](crate) crate
pub type Result<T = (), E = Error> = result::Result<T, E>;

//! Handles errors, with a custom [`Result`] and [`Error`] type

use core::result;

use crate::{credentials, fetch};

/// Errors that may occur while running the app.
#[derive(Debug)]
pub enum Error {
    /// `dotenv` failed to read the `.env` file.
    Credentials(credentials::Error),
    /// Failure occurred while interaction with the IMAP protocol.
    ImapConnection(fetch::connection::Error),
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

/// Overloaded result for the [`mailbox`](crate) crate
pub type Result<T = (), E = Error> = result::Result<T, E>;

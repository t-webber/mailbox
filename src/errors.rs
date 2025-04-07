//! Handles errors, with a custom [`Result`] and [`Error`] type

use core::result;

use crate::{credentials, imap_connection};

/// Errors that may occur while running the app.
#[derive(Debug)]
pub enum Error {
    /// Failure occurred while interaction with the IMAP protocol.
    ImapError(imap_connection::Error),
    /// `dotenv` failed to read the `.env` file.
    InvalidCredentials(credentials::Error),
}

impl From<credentials::Error> for Error {
    fn from(error: credentials::Error) -> Self {
        Self::InvalidCredentials(error)
    }
}

impl From<imap_connection::Error> for Error {
    fn from(error: imap_connection::Error) -> Self {
        Self::ImapError(error)
    }
}

/// Overloaded result for the [`mailbox`](crate) crate
pub type Result<T = (), E = Error> = result::Result<T, E>;

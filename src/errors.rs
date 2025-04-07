//! Handles errors, with a custom [`Result`] and [`Error`] type

use core::result;

use crate::credentials::CredentialsError;

/// Errors that may occur while running the app.
#[derive(Debug)]
pub enum Error {
    /// `dotenv` failed to read the `.env` file.
    InvalidCredentials(CredentialsError),
}

impl From<CredentialsError> for Error {
    fn from(value: CredentialsError) -> Self {
        Self::InvalidCredentials(value)
    }
}

/// Overloaded result for the [`mailbox`](crate) crate
pub type Result<T = (), E = Error> = result::Result<T, E>;

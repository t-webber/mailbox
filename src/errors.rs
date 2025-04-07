//! Handles errors, with a custom [`Result`] and [`Error`] type

use std::env::VarError;
use std::result;

/// Errors that may occur while running the app.
#[derive(Debug)]
pub enum Error {
    /// `dotenv` failed to read the `.env` file.
    CredentialsInvalidFile(dotenv::Error),
    /// The wanted variable is missing in the `.env` file.
    CredentialsMissingVariable(VarError, &'static str),
}

/// Overloaded result for the [`mailbox`](crate) crate
pub type Result<T = (), E = Error> = result::Result<T, E>;

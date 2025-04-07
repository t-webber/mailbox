//! Handles the credentials, by loading them from the `.env` file.
//!
//! You should have a `.env` file at the root with the following variables:
//!
//! ```env
//! DOMAIN=example.com
//! EMAIL=bob@example.com
//! PASSWORD=P@ssw0rd
//! ```

use core::num::ParseIntError;
use std::env::{VarError, var};

use dotenv::dotenv;

use crate::errors::Result;

/// Credentials to interact with the email.
///
/// These credentials should be stored in the `.env` file.
#[derive(Debug)]
pub struct Credentials {
    /// Email domain
    domain: String,
    /// Email
    email: String,
    /// Imap encryption protocol
    ///
    /// # Examples
    ///
    /// `SSL`, `TLS`, etc.
    imap_encryption_protocol: String,
    /// Imap port
    ///
    /// This is set to 993 if none were provided.
    imap_port: u16,
    /// Email password
    password: String,
}

impl Credentials {
    /// Key id for the domain variable in the `.env` file.
    const DOMAIN: &'static str = "DOMAIN";
    /// Key id for the email variable in the `.env` file.
    const EMAIL: &'static str = "EMAIL";
    /// Key id for the imap encryption variable in the `.env` file.
    const IMAP_ENCRYPTION_PROTOCOL: &'static str = "IMAP_ENCRYPTION_PROTOCOL";
    /// Key id for the imap port variable in the `.env` file.
    const IMAP_PORT: &'static str = "IMAP_PORT";
    /// Default imap port.
    const IMAP_PORT_DEFAULT: u16 = 993;
    /// Key id for the password variable in the `.env` file.
    const PASSWORD: &'static str = "PASSWORD";

    /// Returns the domain
    pub fn as_domain(&self) -> &str {
        &self.domain
    }

    /// Loads the credentials from the `.env` file.
    pub fn load() -> Result<Self, CredentialsError> {
        dotenv().map_err(CredentialsError::InvalidFile)?;

        let domain = Self::load_var(Self::DOMAIN)?;
        let email = Self::load_var(Self::EMAIL)?;
        let imap_port = Self::load_imap_port()?;
        let imap_encryption_protocol = Self::load_var(Self::IMAP_ENCRYPTION_PROTOCOL)?;
        let password = Self::load_var(Self::PASSWORD)?;

        Ok(Self { domain, email, imap_encryption_protocol, imap_port, password })
    }

    /// Load the imap port from the `.env`
    ///
    /// Port defaults to [`IMAP_PORT_DEFAULT`](Self::IMAP_PORT_DEFAULT) if it is
    /// not specified.
    fn load_imap_port() -> Result<u16, CredentialsError> {
        Self::load_var(Self::IMAP_PORT).map_or_else(
            |_| Ok(Self::IMAP_PORT_DEFAULT),
            |value| value.parse().map_err(CredentialsError::InvalidPort),
        )
    }

    /// Loads one variable from the `.env` file.
    fn load_var(var_key: &'static str) -> Result<String, CredentialsError> {
        var(var_key).map_err(|err| CredentialsError::MissingVariable(err, var_key))
    }
}

/// Errors that may occur while running the app.
#[derive(Debug)]
pub enum CredentialsError {
    /// `dotenv` failed to read the `.env` file.
    InvalidFile(dotenv::Error),
    /// The provided IMAP port is invalid
    InvalidPort(ParseIntError),
    /// The wanted variable is missing in the `.env` file.
    MissingVariable(VarError, &'static str),
}

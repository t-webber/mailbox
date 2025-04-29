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

pub struct Credentials {
    /// Email domain
    domain_name: String,
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
    pub fn as_domain_name(&self) -> &str {
        &self.domain_name
    }

    /// Returns the email
    pub fn as_email(&self) -> &str {
        &self.email
    }

    /// Returns the socket address
    ///
    /// A socket address is the combination of a hostname and a port.
    pub fn as_imap_socket_address(&self) -> (&str, u16) {
        (&self.domain_name, self.imap_port)
    }

    /// Returns the password
    pub fn as_password(&self) -> &str {
        &self.password
    }

    /// Loads the credentials from the `.env` file.
    pub fn load() -> Result<Self, Error> {
        dotenv().map_err(Error::InvalidFile)?;

        let domain_name = Self::load_var(Self::DOMAIN)?;
        let email = Self::load_var(Self::EMAIL)?;
        let imap_port = Self::load_imap_port()?;
        let imap_encryption_protocol =
            Self::load_var(Self::IMAP_ENCRYPTION_PROTOCOL)?;
        let password = Self::load_var(Self::PASSWORD)?;

        Ok(Self {
            domain_name,
            email,
            imap_encryption_protocol,
            imap_port,
            password,
        })
    }

    /// Load the imap port from the `.env`
    ///
    /// Port defaults to [`IMAP_PORT_DEFAULT`](Self::IMAP_PORT_DEFAULT) if it is
    /// not specified.
    fn load_imap_port() -> Result<u16, Error> {
        Self::load_var(Self::IMAP_PORT).map_or_else(
            |_| Ok(Self::IMAP_PORT_DEFAULT),
            |value| value.parse().map_err(Error::InvalidPort),
        )
    }

    /// Loads one variable from the `.env` file.
    fn load_var(var_key: &'static str) -> Result<String, Error> {
        var(var_key).map_err(|err| Error::MissingVariable(err, var_key))
    }
}

/// Errors that may occur while running the app.
#[derive(Debug)]
pub enum Error {
    /// `dotenv` failed to read the `.env` file.
    InvalidFile(dotenv::Error),
    /// The provided IMAP port is invalid
    InvalidPort(ParseIntError),
    /// The wanted variable is missing in the `.env` file.
    MissingVariable(VarError, &'static str),
}

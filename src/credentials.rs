//! Handles the credentials, by loading them from the `.env` file.
//!
//! You should have a `.env` file at the root with the following variables:
//!
//! ```env
//! DOMAIN=example.com
//! EMAIL=bob@example.com
//! PASSWORD=P@ssw0rd
//! ```

use std::env::var;

use dotenv::dotenv;

use crate::errors::{Error, Result};

/// Credentials to interact with the email.
///
/// These credentials should be stored in the `.env` file.
#[derive(Debug)]
pub struct Credentials {
    /// Email domain
    domain: String,
    /// Email
    email: String,
    /// Email password
    password: String,
}

impl Credentials {
    /// Key id for the domain variable in the `.env` file.
    const DOMAIN: &'static str = "DOMAIN";
    /// Key id for the email variable in the `.env` file.
    const EMAIL: &'static str = "EMAIL";
    /// Key id for the password variable in the `.env` file.
    const PASSWORD: &'static str = "PASSWORD";

    /// Loads the credentials from the `.env` file.
    pub fn load() -> Result<Self> {
        dotenv().map_err(Error::CredentialsInvalidFile)?;

        let domain = Self::load_var(Self::DOMAIN)?;
        let email = Self::load_var(Self::EMAIL)?;
        let password = Self::load_var(Self::PASSWORD)?;

        Ok(Self { domain, email, password })
    }

    /// Loads one variable from the `.env` file.
    fn load_var(var_key: &'static str) -> Result<String> {
        var(var_key).map_err(|err| Error::CredentialsMissingVariable(err, var_key))
    }
}

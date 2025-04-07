//! Handles the IMAP connections.
//!
//! IMAP is the protocol responsible for fetching emails. This will allow
//! [`mailbox`](crate) to read the emails.

use core::str::{Utf8Error, from_utf8};
use std::net;

use native_tls::TlsConnector;

use crate::credentials::Credentials;
use crate::errors::Result;

/// Represents the Imap session to communicate with the server.
pub struct ImapSession {
    /// Active session
    session: imap::Session<native_tls::TlsStream<net::TcpStream>>,
}

impl ImapSession {
    /// Returns the body of the latest email in the `INBOX` folder.
    pub fn get_latest_inbox_mail(&mut self) -> Result<String> {
        self.session
            .select("INBOX")
            .map_err(Error::InvalidMailboxName)?;

        let mails = self
            .session
            .fetch("1", "RFC822")
            .map_err(Error::ImapFetch)?;
        let first_mail = mails.into_iter().next().ok_or(Error::NoEmail)?;

        let first_mail_utf8_body = first_mail.body().ok_or(Error::NoBody)?;
        let first_mail_body = from_utf8(first_mail_utf8_body).map_err(Error::InvalidBody)?;

        Ok(first_mail_body.to_owned())
    }

    /// Creates a new [`ImapSession`] with the given [`Credentials`].
    pub fn with_credentials(credentials: &Credentials) -> Result<Self> {
        let socket_address = credentials.as_imap_socket_address();
        let domain_name = credentials.as_domain_name();
        let ssl_connector = TlsConnector::new().map_err(Error::TlsConnection)?;

        let client = imap::connect(socket_address, domain_name, &ssl_connector)
            .map_err(Error::ImapConnection)?;

        let session = client
            .login(credentials.as_email(), credentials.as_password())
            .map_err(|(err, _)| Error::ImapConnection(err))?;

        Ok(Self { session })
    }
}

impl Drop for ImapSession {
    fn drop(&mut self) {
        #[expect(clippy::print_stderr, reason = "Err not possible in drop")]
        if let Err(err) = self.session.logout() {
            eprintln!("{err}\nFailed to log out from session. May still be active.");
        }
    }
}

/// Errors that may occur while interaction in `IMAP`.
#[derive(Debug)]
pub enum Error {
    /// Failed to connect to the IMAP server.
    ImapConnection(imap::Error),
    /// Failed to fetch from the IMAP server.
    ImapFetch(imap::Error),
    /// Given email has an invalid body.
    InvalidBody(Utf8Error),
    /// Failed to read the wanted mailbox name.
    InvalidMailboxName(imap::Error),
    /// Given email has no body
    NoBody,
    /// No emails were found with the given requirements.
    NoEmail,
    /// Failed to establish `TLS` connection.
    TlsConnection(native_tls::Error),
}

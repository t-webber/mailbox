//! Handles the IMAP connections.
//!
//! IMAP is the protocol responsible for fetching emails. This will allow
//! [`mailbox`](crate) to read the emails.

use core::marker::PhantomData;
use core::str::{Utf8Error, from_utf8};
use std::net;

use imap::types::Fetch;
use native_tls::TlsConnector;

use crate::credentials::Credentials;
use crate::errors::Result;

/// Type of query made on the IMAP server.
const QUERY: &str = "RFC822";

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

/// Represents the Imap session to communicate with the server.
pub struct ImapSession<T> {
    /// Marker to keep the status of the [`ImapSession`]
    ///
    /// This is a zero-sized element that informs on whether a mailbox was
    /// specified or not
    marker: PhantomData<T>,
    /// Active session
    session: imap::Session<native_tls::TlsStream<net::TcpStream>>,
}

impl ImapSession<None> {
    /// Selects a mailbox to fetch
    pub fn select_mailbox(
        mut self,
        mailbox_name: &str,
    ) -> Result<ImapSession<MailboxSelected>> {
        self.session
            .select(mailbox_name)
            .map_err(Error::InvalidMailboxName)?;
        Ok(ImapSession { session: self.session, marker: PhantomData })
    }

    /// Creates a new [`ImapSession`] with the given [`Credentials`].
    pub fn with_credentials(credentials: &Credentials) -> Result<Self> {
        let socket_address = credentials.as_imap_socket_address();
        let domain_name = credentials.as_domain_name();
        let ssl_connector =
            TlsConnector::new().map_err(Error::TlsConnection)?;

        let client = imap::connect(socket_address, domain_name, &ssl_connector)
            .map_err(Error::ImapConnection)?;

        let session = client
            .login(credentials.as_email(), credentials.as_password())
            .map_err(|(err, _)| Error::ImapConnection(err))?;

        Ok(Self { session, marker: PhantomData })
    }
}

impl ImapSession<MailboxSelected> {
    /// Get all the emails of the chosen mailbox.
    pub fn get_all_mails(&mut self) -> Result<Vec<String>> {
        self.session
            .fetch("1:*", QUERY)
            .map_err(Error::ImapFetch)?
            .into_iter()
            .map(get_email_body)
            .collect()
    }

    /// Returns an email from its unique id.
    pub fn get_mail_from_uid(&mut self, uid: u32) -> Result<String> {
        let response = self
            .session
            .uid_fetch(uid.to_string(), QUERY)
            .map_err(Error::ImapFetch)?;
        let mail = response.first().ok_or(Error::NoEmail)?;
        get_email_body(mail)
    }

    /// Returns the body of the latest email in the `INBOX` folder.
    pub fn get_uids(&mut self) -> Result<Vec<u32>> {
        let mut uids = self
            .session
            .uid_search("ALL")
            .map_err(Error::ImapFetch)?
            .into_iter()
            .collect::<Vec<_>>();
        uids.sort_unstable();
        uids.reverse();
        Ok(uids)
    }
}

/// State of the [`ImageSession`] after a session was created.
pub struct MailboxSelected;

/// State of the [`ImageSession`] before a session was created.
pub struct None;

/// Get the body of an email
///
/// The body of an email also contains all the headers.
fn get_email_body(mail: &Fetch) -> Result<String> {
    let body = mail.body().ok_or(Error::NoBody)?;
    Ok(from_utf8(body).map_err(Error::InvalidBody)?.to_owned())
}

#[cfg(test)]
mod test {

    use crate::credentials::Credentials;
    use crate::errors::Result;
    use crate::fetch::connection::ImapSession;

    #[expect(
        clippy::panic_in_result_fn,
        clippy::unwrap_used,
        reason = "test and system failure"
    )]
    #[test]
    fn check_first_last() -> Result {
        let credentials = Credentials::load()?;
        let imap_session = ImapSession::with_credentials(&credentials)?;
        let mut inbox = imap_session.select_mailbox("INBOX")?;

        let mails = inbox.get_all_mails()?;
        let first_all = mails.first().unwrap();
        let last_all = mails.last().unwrap();

        let mut uids = inbox.get_uids()?.into_iter().collect::<Vec<_>>();
        uids.sort_unstable();

        let first_single = inbox.get_mail_from_uid(*uids.first().unwrap())?;
        let last_single = inbox.get_mail_from_uid(*uids.last().unwrap())?;

        assert!(first_all == &first_single);
        assert!(last_all == &last_single);

        Ok(())
    }
}

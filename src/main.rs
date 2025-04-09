//! Rust crate to load emails with IMAP protocol and send emails with SMTP
//! protocol.

#![warn(
    missing_docs,
    warnings,
    deprecated_safe,
    future_incompatible,
    keyword_idents,
    let_underscore,
    nonstandard_style,
    refining_impl_trait,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    rust_2024_compatibility,
    unused,
    clippy::all,
    clippy::pedantic,
    clippy::style,
    clippy::perf,
    clippy::complexity,
    clippy::correctness,
    clippy::restriction,
    clippy::nursery
)]
#![expect(clippy::blanket_clippy_restriction_lints, reason = "enable all lints")]
#![expect(
    clippy::single_call_fn,
    clippy::implicit_return,
    clippy::question_mark_used,
    reason = "bad lint"
)]
#![expect(clippy::mod_module_files, reason = "chosen style")]
#![expect(dead_code, reason = "implementation in progress")]
#![allow(clippy::arbitrary_source_item_ordering, reason = "issue #14570")]

mod credentials;
mod errors;
mod fetch;

const fn main() {}

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

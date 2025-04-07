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
#![expect(dead_code, reason = "implementation in progress")]

mod credentials;
mod errors;
mod imap_connection;
use credentials::Credentials;
use errors::Result;
use imap_connection::ImapSession;

#[expect(clippy::dbg_macro, reason = "debugging")]
fn main() -> Result {
    let credentials = dbg!(Credentials::load()?);
    let imap_session = ImapSession::with_credentials(&credentials);
    Ok(())
}

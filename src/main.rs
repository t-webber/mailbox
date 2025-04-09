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
#![allow(clippy::pattern_type_mismatch, reason = "conveniant")]

mod credentials;
mod errors;
mod fetch;
mod tui;

fn main() -> errors::Result {
    tui::Tui::new().run()
}

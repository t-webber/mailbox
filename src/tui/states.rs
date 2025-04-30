//! File to handle the states
//!
//! A user changes states by using the buttons at the top of the UI, or with the
//! associated keybindings.

use super::writer::Writer;

/// Current mode of the TUI, specifying what is the user doing
#[derive(Default)]
pub enum TuiMode {
    /// Display the help window, with different keybindings
    #[default]
    Help,
    /// Displaying emails to read different inboxes
    Reading,
    /// Writing an email
    Writing(Writer),
}

impl TuiMode {
    /// Switch to writer mode
    ///
    /// This creates a default writer and opens it in the TUI app.
    pub fn new_writer(&mut self) {
        *self = Self::Writing(Writer::default());
    }
}

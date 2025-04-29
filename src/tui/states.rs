//! File to handle the states
//!
//! A user changes states by using the buttons at the top of the UI, or with the
//! associated keybindings.

/// Current mode of the TUI, specifying what is the user doing
#[derive(Default)]
pub enum TuiMode {
    /// Display the help window, with different keybindings
    #[default]
    Help,
    /// Displaying emails to read different inboxes
    Reading,
    /// Writing an email
    Writing,
}

use ratatui::Frame;
use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::text::Text;
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler as _;

/// Representation of the writer, with the different boxes.
#[derive(Default)]
pub struct Writer {
    /// Input to enter the subject of the email
    subject: Input,
    /// Input to enter the destination(s) of the email
    ///
    /// If there are multiple destinations, they must be seperated with a
    /// comma. Spaces are ignored.
    to: Input,
    /// Input to enter the body of the email
    body: Input,
    /// State of the writer
    ///
    /// Specifies what Input is being edited
    state: WriterState,
}

impl Writer {
    /// Main method to display the layout on every re-render of the page
    #[expect(clippy::indexing_slicing, reason = "constant size and indexes")]
    pub fn writer_page(&self, frame: &mut Frame<'_>) {
        let layout = Layout::new(
            Direction::Vertical,
            [Constraint::Max(5), Constraint::Max(5), Constraint::Fill(1)],
        )
        .split(frame.area());

        assert!(layout.len() == 3, "Layout has 3 elements");

        frame.render_widget(Text::from("hello wrold"), frame.area());
        frame.render_widget(self.subject.value(), layout[0]);
        frame.render_widget(self.to.value(), layout[1]);
        frame.render_widget(self.body.value(), layout[2]);
        frame.render_widget(Text::from("hello wrolu2"), frame.area());
    }

    /// Handler to manage keypresses.
    pub fn handle_key_events(&mut self, event: &Event) -> bool {
        if let Event::Key(key) = event {
            match (&self.state, key.code) {
                (WriterState::None, KeyCode::Char('t')) =>
                    self.state = WriterState::To,
                (WriterState::None, KeyCode::Char('s')) =>
                    self.state = WriterState::Subject,
                (WriterState::None, KeyCode::Char('b')) =>
                    self.state = WriterState::Body,
                (
                    WriterState::To | WriterState::Subject | WriterState::Body,
                    KeyCode::Esc,
                ) => self.state = WriterState::None,
                (WriterState::Subject, _) => {
                    self.subject.handle_event(event);
                }
                (WriterState::Body, _) => {
                    self.body.handle_event(event);
                }
                (WriterState::To, _) => {
                    self.to.handle_event(event);
                }
                _ => return false,
            }
        }
        true
    }
}

/// State of the writer, informing on which input is being edited by the client.
#[derive(Default)]
enum WriterState {
    /// No Input is being edited.
    ///
    /// Press `Esc` to enter this mode.
    #[default]
    None,
    /// The destination input is being edited.
    ///
    /// Press `t` to enter this mode.
    To,
    /// The subject input is being edited.
    ///
    /// Press `s` to enter this mode.
    Subject,
    /// The body input is being edited.
    ///
    /// Press `b` to enter this mode.
    Body,
}

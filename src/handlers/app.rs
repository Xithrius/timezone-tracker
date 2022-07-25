use rustyline::line_buffer::LineBuffer;
use tui::style::Style;

use crate::{
    handlers::storage::Storage,
    utils::{pathing::config_path, styles},
};

pub enum State {
    Normal,
    Input,
    // Help,
}

pub struct App {
    /// State of the application.
    pub state: State,
    /// Storing information of timezones in a file.
    pub storage: Storage,
    /// The single box for inserting information into.
    pub input_buffer: LineBuffer,
    /// The different validity states that the input buffer contains.
    pub buffer_validity: Style,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: State::Normal,
            storage: Storage::new(config_path("storage.json")),
            input_buffer: LineBuffer::with_capacity(4096),
            buffer_validity: styles::COLUMN_TITLE,
        }
    }

    pub fn cleanup(&self) {
        self.storage.dump_data();
    }
}

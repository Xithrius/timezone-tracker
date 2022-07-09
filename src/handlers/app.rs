use rustyline::line_buffer::LineBuffer;

use crate::{handlers::storage::Storage, utils::pathing::config_path};

pub enum State {
    Normal,
    Input,
    // Help,
}

pub struct App {
    /// State of the application
    pub state: State,
    /// Storing information of timezones in a file.
    pub storage: Storage,
    /// The single box for inserting information into
    pub input_buffer: LineBuffer,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: State::Normal,
            storage: Storage::new(config_path("storage.json")),
            input_buffer: LineBuffer::with_capacity(4096),
        }
    }

    pub fn cleanup(&self) {
        self.storage.dump_data();
    }
}

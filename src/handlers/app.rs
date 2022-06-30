use std::path::PathBuf;

use color_eyre::eyre::{bail, Error, Result};
use rusqlite::Connection;
use rustyline::line_buffer::LineBuffer;

use crate::{handlers::database::Database, utils::pathing::config_path};

#[allow(dead_code)]
pub enum State {
    Normal,
    Input,
    Help,
}

pub struct App {
    /// State of the application
    pub state: State,
    /// Users and their timezone offset
    pub database: Database,
    /// The single box for inserting information into
    pub input_buffer: LineBuffer,
}

impl App {
    pub fn new() -> Result<Self, Error> {
        let database_path = config_path("db.sqlite3");

        match Connection::open(&database_path) {
            Ok(database_connection) => Ok(Self {
                state: State::Normal,
                database: Database::new(database_connection),
                input_buffer: LineBuffer::with_capacity(4096),
            }),
            Err(_) => bail!(rusqlite::Error::InvalidPath(PathBuf::from(&database_path))),
        }
    }
}

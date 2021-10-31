use std::{
    collections::{HashMap, VecDeque},
    path::PathBuf,
};

use anyhow::{bail, Error, Result};
use maplit::hashmap;
use rusqlite::Connection;
use rustyline::line_buffer::LineBuffer;

use crate::utils::pathing::config_path;

pub enum State {
    Normal,
    Input,
    Help,
}

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub offset: i64,
}

impl User {
    pub fn new(name: String, offset: i64) -> Self {
        Self { name, offset }
    }
}

pub struct App {
    /// State of the application
    pub state: State,
    /// Users and their timezone offset
    pub timezone_data: VecDeque<User>,
    /// Connection to the sqlite database
    pub conn: Connection,
    /// Boxes to insert text into
    pub input_map: HashMap<&'static str, LineBuffer>,
}

impl App {
    pub fn new() -> Result<Self, Error> {
        let database_path = config_path("db.sqlite3");

        match Connection::open(&database_path) {
            Ok(database_connection) => Ok(Self {
                timezone_data: VecDeque::new(),
                conn: database_connection,
                state: State::Normal,
                input_map: hashmap! {"timezone" => LineBuffer::with_capacity(4096)},
            }),
            Err(_) => bail!(rusqlite::Error::InvalidPath(PathBuf::from(&database_path))),
        }
    }
}

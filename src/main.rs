use std::{io, time::Duration};

use anyhow::Result;
use rusqlite::Connection;
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Row, Table},
    Terminal,
};

use crate::utils::{
    app::{App, User},
    event,
};

mod utils;

// const DEFAULT_DB_PATH_LINUX = &str = "~/.local/share/timezone-tracker/db.sqlite3";
const DB_PATH: &str = "./db.sqlite3";

fn main() -> Result<()> {
    let events = event::Events::with_config(event::Config {
        exit_key: Key::Esc,
        tick_rate: Duration::from_millis(30),
    });

    let mut app = App::new();

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let window_constraints = [Constraint::Length(3), Constraint::Min(1)];
    let table_columns = vec!["User", "Offset", "Time"];

    let conn = Connection::open(&DB_PATH).unwrap();

    conn.execute(
        "create table if not exists users (
            id integer primary key,
            name text not null unique,
            offset integer3
        )",
        [],
    )
    .unwrap();

    let mut stmt = conn.prepare("SELECT name, offset FROM users").unwrap();

    let query = stmt
        .query_map([], |row| {
            Ok(User {
                name: row.get(0).unwrap(),
                offset: row.get(1).unwrap(),
            })
        })
        .unwrap();

    for user in query {
        app.timezone_data.push_front(user.unwrap());
    }

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(window_constraints.as_ref())
                .split(f.size());

            let block = Block::default().title("Block").borders(Borders::ALL);

            f.render_widget(block, chunks[0]);

            let table = Table::new(vec![Row::new(vec!["one", "two", "three"])])
                .header(Row::new(table_columns.clone()))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("[ Timezones ]"),
                )
                .widths([Constraint::Length(15), Constraint::Length(15)].as_ref())
                .column_spacing(1);

            f.render_widget(table, chunks[1]);
        })?;

        if let event::Event::Input(input) = events.next()? {
            match input {
                Key::Char('q') | Key::Esc => break,
                _ => {}
            }
        }
    }

    Ok(())
}

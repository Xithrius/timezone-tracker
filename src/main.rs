use std::{
    io::{self, Stdout},
    time::Duration,
};

use anyhow::Result;
use chrono::Local;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rusqlite::Connection;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Row, Table},
    Terminal,
};

use crate::utils::{
    app::{App, User},
    event,
};

mod utils;

const DB_PATH: &str = "./db.sqlite3";

fn main() -> Result<()> {
    let events = event::Events::with_config(event::Config {
        exit_key: KeyCode::Null,
        tick_rate: Duration::from_millis(30),
    });

    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend).unwrap();

    let mut app = App::new();

    // let window_constraints = [Constraint::Length(3), Constraint::Min(1)];
    let window_constraints = [Constraint::Min(1)];
    let table_columns = vec!["User", "Offset", "Time"];

    let conn = Connection::open(&DB_PATH).unwrap();

    conn.execute(
        "create table if not exists users (
            id integer primary key,
            name text not null unique,
            offset integer
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

    terminal.clear().unwrap();

    let quitting = |mut terminal: Terminal<CrosstermBackend<Stdout>>| {
        disable_raw_mode().unwrap();
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .unwrap();
        terminal.show_cursor().unwrap();
    };

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(window_constraints.as_ref())
                .split(f.size());

            let table = Table::new(vec![Row::new(vec!["one", "two", "three"])])
                .header(Row::new(table_columns.clone()))
                .block(Block::default().borders(Borders::ALL).title(format!(
                    "[ Timezones Table ] [ Local time: {} ]",
                    Local::now().format("%c").to_string()
                )))
                .widths([Constraint::Length(15), Constraint::Length(15)].as_ref())
                .column_spacing(1);

            f.render_widget(table, chunks[0]);
        })?;

        if let event::Event::Input(input) = events.next()? {
            match input.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    quitting(terminal);
                    break;
                }
                _ => {}
            }
        }
    }

    Ok(())
}

use std::{
    collections::VecDeque,
    io::{self, Stdout},
    time::Duration,
};

use crate::{
    handlers::{
        app::{App, State, User},
        config::CompleteConfig,
        event::{self, Event, Key},
    },
    utils::text::{align_text, get_cursor_position, parse_timezone_offset},
};
use chrono::{Local, NaiveDateTime, Utc};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rusqlite::params;
use rustyline::{At, Word};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Row, Table},
    Terminal,
};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

pub async fn draw_terminal_ui(config: &CompleteConfig) {
    let mut events = event::Events::with_config(event::Config {
        exit_key: Key::Null,
        tick_rate: Duration::from_millis(100),
    })
    .await;

    enable_raw_mode().unwrap();

    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend).unwrap();

    let mut app = App::new().expect("Unsuccessful in finding file/folder of database.");

    let table_columns = vec!["User", "Offset", "Time"];

    app.conn
        .execute(
            "create table if not exists users (
            id integer primary key,
            name text not null unique,
            timezone_offset integer
        )",
            [],
        )
        .expect("Failed to create database table");

    let mut stmt = app
        .conn
        .prepare("SELECT name, timezone_offset FROM users")
        .unwrap();

    app.timezone_data = stmt
        .query_map([], |row| {
            Ok(User {
                name: row.get(0).unwrap(),
                offset: row.get(1).unwrap(),
            })
        })
        .unwrap()
        .filter_map(|f| f.ok())
        .collect::<VecDeque<User>>();

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

    'outer: loop {
        terminal
            .draw(|f| {
                let mut vertical_chunk_constraints = vec![Constraint::Min(1)];

                if let State::Input = app.state {
                    vertical_chunk_constraints.push(Constraint::Length(3));
                }

                let vertical_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(vertical_chunk_constraints.as_ref())
                    .split(f.size());

                let row_times = app
                    .timezone_data
                    .iter()
                    .map(|u| {
                        Row::new(vec![
                            u.name.clone(),
                            align_text(format!("{}", u.offset.clone()).as_str(), "right", 6),
                            NaiveDateTime::from_timestamp(
                                Utc::now().timestamp() + u.offset * 3600,
                                0,
                            )
                            .format(config.local_time_format.as_str())
                            .to_string(),
                        ])
                    })
                    .collect::<Vec<Row>>();

                let table = Table::new(row_times)
                    .header(Row::new(table_columns.clone()))
                    .block(Block::default().borders(Borders::ALL).title(format!(
                        "[ Timezones Table ] [ Local time: {} ]",
                        Local::now().format(config.local_time_format.as_str()).to_string()
                    )))
                    .widths(
                        [
                            Constraint::Length(15),
                            Constraint::Length(6),
                            Constraint::Percentage(100),
                        ]
                        .as_ref(),
                    )
                    .column_spacing(1);

                f.render_widget(table, vertical_chunks[0]);

                if let State::Input = app.state {
                    let text = app
                        .input_map
                        .get("timezone")
                        .expect("Could not find timezone input window");

                    let cursor_pos = get_cursor_position(text);

                    let input_rect = vertical_chunks[vertical_chunk_constraints.len() - 1];

                    f.set_cursor(
                        (input_rect.x + cursor_pos as u16 + 1)
                            .min(input_rect.x + input_rect.width.saturating_sub(2)),
                        input_rect.y + 1,
                    );

                    let paragraph = Paragraph::new(text.as_str())
                        .style(Style::default().fg(Color::Yellow))
                        .block(Block::default().borders(Borders::ALL).title("[ Input ]"))
                        .scroll((
                            0,
                            ((cursor_pos + 3) as u16).saturating_sub(input_rect.width),
                        ));

                    f.render_widget(
                        paragraph,
                        vertical_chunks[vertical_chunk_constraints.len() - 1],
                    );
                }
            })
            .unwrap();

        if let Some(Event::Input(key)) = events.next().await {
            match app.state {
                State::Normal => match key {
                    Key::Esc => {
                        quitting(terminal);
                        break 'outer;
                    }
                    Key::Char('i') => {
                        app.state = State::Input;
                    }
                    _ => {}
                },
                State::Input => {
                    let timezone = app.input_map.get_mut("timezone").unwrap();

                    match key {
                        Key::Ctrl('f') | Key::Right => {
                            timezone.move_forward(1);
                        }
                        Key::Ctrl('b') | Key::Left => {
                            timezone.move_backward(1);
                        }
                        Key::Ctrl('a') | Key::Home => {
                            timezone.move_home();
                        }
                        Key::Ctrl('e') | Key::End => {
                            timezone.move_end();
                        }
                        Key::Alt('f') => {
                            timezone.move_to_next_word(At::AfterEnd, Word::Emacs, 1);
                        }
                        Key::Alt('b') => {
                            timezone.move_to_prev_word(Word::Emacs, 1);
                        }
                        Key::Ctrl('t') => {
                            timezone.transpose_chars();
                        }
                        Key::Alt('t') => {
                            timezone.transpose_words(1);
                        }
                        Key::Ctrl('u') => {
                            timezone.discard_line();
                        }
                        Key::Ctrl('k') => {
                            timezone.kill_line();
                        }
                        Key::Ctrl('w') => {
                            timezone.delete_prev_word(Word::Emacs, 1);
                        }
                        Key::Ctrl('d') => {
                            timezone.delete(1);
                        }
                        Key::Backspace | Key::Delete => {
                            timezone.backspace(1);
                        }
                        Key::Enter => {
                            let input_message = timezone.as_str();

                            if !input_message.is_empty() {
                                // This is temporary. There will be multiple text boxes in the future to do this.
                                let data = input_message.split(',').collect::<Vec<&str>>();

                                if let Ok(timezone_offset) = parse_timezone_offset(data[1]) {
                                    app.timezone_data.push_front(User::new(
                                        data[0].to_string(),
                                        timezone_offset,
                                    ));

                                    app.conn.execute(
                                        "INSERT INTO users (name, timezone_offset) VALUES (?1, ?2)",
                                        params![data[0], timezone_offset],
                                    ).expect("Failed to insert timezone data into database.");

                                    timezone.update("", 0);
                                }
                            }
                        }
                        Key::Char(c) => {
                            timezone.insert(c, 1);
                        }
                        Key::Esc => {
                            timezone.update("", 0);
                            app.state = State::Normal;
                        }
                        _ => {}
                    }
                }
                State::Help => {}
            }
        }
    }
}

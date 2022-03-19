use std::{
    io::{self, Stdout},
    time::Duration,
};

use chrono::{Local, NaiveDateTime, Utc};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rustyline::{At, Word};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Row, Table},
    Terminal,
};

use crate::{
    handlers::{
        app::{App, State},
        config::CompleteConfig,
        event::{self, Event, Key},
    },
    utils::{
        styles,
        text::{align_text, get_cursor_position, parse_timezone_offset},
    },
};

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
                    .database
                    .users
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
                    .header(Row::new(table_columns.clone()).style(styles::COLUMN_TITLE))
                    .block(
                        Block::default()
                            .style(styles::BORDER_NAME)
                            .borders(Borders::ALL)
                            .title(format!(
                                "[ Timezones Table ] [ Local time: {} ]",
                                Local::now().format(config.local_time_format.as_str())
                            )),
                    )
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
                    let text = &app.input_buffer;

                    let cursor_pos = get_cursor_position(text);

                    let input_rect = vertical_chunks[vertical_chunk_constraints.len() - 1];

                    f.set_cursor(
                        (input_rect.x + cursor_pos as u16 + 1)
                            .min(input_rect.x + input_rect.width.saturating_sub(2)),
                        input_rect.y + 1,
                    );

                    let paragraph = Paragraph::new(text.as_str())
                        .style(Style::default().fg(Color::Yellow))
                        .block(
                            Block::default()
                                .style(styles::BORDER_NAME)
                                .borders(Borders::ALL)
                                .title("[ Input ]"),
                        )
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
                    match key {
                        Key::Ctrl('f') | Key::Right => {
                            app.input_buffer.move_forward(1);
                        }
                        Key::Ctrl('b') | Key::Left => {
                            app.input_buffer.move_backward(1);
                        }
                        Key::Ctrl('a') | Key::Home => {
                            app.input_buffer.move_home();
                        }
                        Key::Ctrl('e') | Key::End => {
                            app.input_buffer.move_end();
                        }
                        Key::Alt('f') => {
                            app.input_buffer
                                .move_to_next_word(At::AfterEnd, Word::Emacs, 1);
                        }
                        Key::Alt('b') => {
                            app.input_buffer.move_to_prev_word(Word::Emacs, 1);
                        }
                        Key::Ctrl('t') => {
                            app.input_buffer.transpose_chars();
                        }
                        Key::Alt('t') => {
                            app.input_buffer.transpose_words(1);
                        }
                        Key::Ctrl('u') => {
                            app.input_buffer.discard_line();
                        }
                        Key::Ctrl('k') => {
                            app.input_buffer.kill_line();
                        }
                        Key::Ctrl('w') => {
                            app.input_buffer.delete_prev_word(Word::Emacs, 1);
                        }
                        Key::Ctrl('d') => {
                            app.input_buffer.delete(1);
                        }
                        Key::Backspace | Key::Delete => {
                            app.input_buffer.backspace(1);
                        }
                        Key::Enter => {
                            let input_message = &app.input_buffer.as_str();

                            if !input_message.is_empty() {
                                // This is temporary. There will be multiple text boxes in the future to do this.
                                let data = input_message.split(',').collect::<Vec<&str>>();

                                if let Ok(timezone_offset) = parse_timezone_offset(data[1]) {
                                    app.database.add(data[0].to_string(), timezone_offset);

                                    app.input_buffer.update("", 0);
                                }
                            }
                        }
                        Key::Char(c) => {
                            app.input_buffer.insert(c, 1);
                        }
                        Key::Esc => {
                            app.input_buffer.update("", 0);
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

use std::{
    io::{stdout, Stdout},
    time::Duration,
};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rustyline::{At, Word};
use tui::{backend::CrosstermBackend, Terminal};

use crate::{
    handlers::{
        app::{App, State},
        config::CompleteConfig,
        event::{self, Event, Key},
    },
    ui::draw_ui,
    utils::timezones::parse_user_timezone,
};

fn reset_terminal() {
    disable_raw_mode().unwrap();

    execute!(stdout(), LeaveAlternateScreen).unwrap();
}

fn init_terminal() -> Terminal<CrosstermBackend<Stdout>> {
    enable_raw_mode().unwrap();

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();

    let backend = CrosstermBackend::new(stdout);

    Terminal::new(backend).unwrap()
}

fn quit_terminal(mut terminal: Terminal<CrosstermBackend<Stdout>>) {
    disable_raw_mode().unwrap();

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .unwrap();

    terminal.show_cursor().unwrap();
}

pub async fn ui_driver(config: CompleteConfig, mut app: App) {
    let original_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic| {
        reset_terminal();
        original_hook(panic);
    }));

    let mut events = event::Events::with_config(event::Config {
        exit_key: Key::Null,
        tick_rate: Duration::from_millis(100),
    });

    let mut terminal = init_terminal();

    terminal.clear().unwrap();

    'outer: loop {
        terminal
            .draw(|frame| draw_ui(frame, &mut app, &config))
            .unwrap();

        if let Some(Event::Input(key)) = events.next().await {
            match app.state {
                State::Normal => match key {
                    Key::Char('q') => {
                        quit_terminal(terminal);

                        break 'outer;
                    }
                    Key::Char('i') => {
                        app.state = State::Input;
                    }
                    _ => {}
                },
                State::Input => match key {
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
                            if let Ok((user, offset)) = parse_user_timezone(input_message) {
                                app.storage.add(&user, offset);

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
                },
            }
        }
    }

    app.cleanup();

    reset_terminal();
}

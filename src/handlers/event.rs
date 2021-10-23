use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event as CEvent, KeyCode, KeyEvent};

pub enum Event<I> {
    Input(I),
    Tick,
}

#[allow(dead_code)]
pub struct Events {
    rx: mpsc::Receiver<Event<KeyEvent>>,
    input_handle: thread::JoinHandle<()>,
    tick_handle: thread::JoinHandle<()>,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub exit_key: KeyCode,
    pub tick_rate: Duration,
}

impl Events {
    pub fn with_config(config: Config) -> Events {
        let (tx, rx) = mpsc::channel();

        let input_handle = {
            let tx = tx.clone();

            thread::spawn(move || {
                let mut last_tick = Instant::now();

                loop {
                    let timeout = config
                        .tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or_else(|| Duration::from_secs(0));

                    if event::poll(timeout).unwrap() {
                        if let Ok(CEvent::Key(key)) = event::read() {
                            if let Err(err) = tx.send(Event::Input(key)) {
                                eprintln!("{}", err);
                                return;
                            }
                        }
                    }

                    if last_tick.elapsed() >= config.tick_rate {
                        if let Err(err) = tx.send(Event::Tick) {
                            eprintln!("{}", err);
                            return;
                        }
                        last_tick = Instant::now();
                    }
                }
            })
        };

        let tick_handle = {
            thread::spawn(move || loop {
                if tx.send(Event::Tick).is_err() {
                    break;
                }
                thread::sleep(config.tick_rate);
            })
        };

        Events {
            rx,
            input_handle,
            tick_handle,
        }
    }

    pub fn next(&self) -> Result<Event<KeyEvent>, mpsc::RecvError> {
        self.rx.recv()
    }
}

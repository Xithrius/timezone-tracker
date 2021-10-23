use handlers::config::CompleteConfig;

mod handlers;
mod terminal;
mod utils;

use anyhow::{Result};

fn main() -> Result<()> {
    match CompleteConfig::new() {
        Ok(config) => terminal::draw_terminal_ui(&config),
        Err(err) => println!("{}", err),
    };

    Ok(())
}

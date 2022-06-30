mod handlers;
mod terminal;
mod utils;

use color_eyre::eyre::{Result, WrapErr};
use handlers::config::CompleteConfig;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install().unwrap();

    let config = CompleteConfig::new()
        .wrap_err("Configuration error.")
        .unwrap();

    terminal::draw_terminal_ui(&config).await;

    Ok(())
}

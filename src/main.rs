use anyhow::{bail, Result};

mod handlers;
mod terminal;
mod utils;

use handlers::config::CompleteConfig;

#[tokio::main]
async fn main() -> Result<()> {
    let config = match CompleteConfig::new() {
        Ok(c) => c,
        Err(err) => bail!("{}", err),
    };

    terminal::draw_terminal_ui(&config).await;

    Ok(())
}

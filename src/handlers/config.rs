use color_eyre::eyre::{bail, Error, Result};
use serde::Deserialize;

use crate::utils::pathing::config_path;

#[derive(Deserialize, Debug)]
pub struct CompleteConfig {
    pub terminal: TerminalConfig,
    pub frontend: FrontendConfig,
}

#[derive(Deserialize, Debug)]
pub struct TerminalConfig {
    /// Your timezone offset. Only use this if the time in the top left isn't correct.
    pub local_timezone_offset: String,
    /// The delay in milliseconds between terminal updates.
    pub tick_delay: usize,
}

#[derive(Deserialize, Debug)]
pub struct FrontendConfig {
    /// The format of the date and time outputs, examples at https://strftime.org/
    pub local_time_format: String,
    /// The longest a username can be.
    pub maximum_username_length: usize,
    /// Which side the username should be aligned to.
    pub username_alignment: String,
    /// Show padding around chat frame
    pub padding: bool,
}

impl CompleteConfig {
    pub fn new() -> Result<Self, Error> {
        if let Ok(config_contents) = std::fs::read_to_string(config_path("config.toml")) {
            let config: CompleteConfig = toml::from_str(config_contents.as_str()).unwrap();

            Ok(config)
        } else {
            bail!(
                "Configuration not found. Create a config file at '{}', and see '{}' for an example configuration.",
                config_path("config.toml"),
                format!("{}/blob/main/default-config.toml", env!("CARGO_PKG_REPOSITORY"))
            )
        }
    }
}

use std::{
    fs::{create_dir_all, read_to_string, File},
    io::Write,
    path::Path,
};

use color_eyre::eyre::{bail, Error, Result};
use serde::{Deserialize, Serialize};

use crate::utils::pathing::config_path;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct CompleteConfig {
    /// Internal functionality.
    pub terminal: TerminalConfig,
    /// How everything looks to the user.
    pub frontend: FrontendConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TerminalConfig {
    /// The delay in milliseconds between terminal updates.
    pub tick_delay: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Alignment {
    Left,
    Right,
    Center,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FrontendConfig {
    /// The format of the date and time outputs. Formats can be found at https://strftime.org/.
    pub time_format: String,
    /// Which side the information should be aligned to.
    pub alignment: Alignment,
    /// The amount of padding that the main window should have.
    pub padding: u16,
}

impl Default for TerminalConfig {
    fn default() -> Self {
        Self { tick_delay: 30 }
    }
}

impl Default for Alignment {
    fn default() -> Self {
        Alignment::Right
    }
}

impl Default for FrontendConfig {
    fn default() -> Self {
        Self {
            time_format: "%c".to_string(),
            alignment: Alignment::Right,
            padding: 0,
        }
    }
}

impl CompleteConfig {
    pub fn new() -> Result<Self, Error> {
        let path_str = config_path("config.toml");

        let p = Path::new(&path_str);

        if !p.exists() {
            create_dir_all(p.parent().unwrap()).unwrap();

            let default_toml_string = toml::to_string(&CompleteConfig::default()).unwrap();
            let mut file = File::create(path_str.clone()).unwrap();
            file.write_all(default_toml_string.as_bytes()).unwrap();

            bail!("Configuration was generated at {path_str}, please fill it out with necessary information.")
        } else if let Ok(config_contents) = read_to_string(&p) {
            let config: CompleteConfig = toml::from_str(config_contents.as_str()).unwrap();

            Ok(config)
        } else {
            bail!(
                "Configuration could not be read correctly. See the following link for the example config: {}",
                format!("{}/blob/main/default-config.toml", env!("CARGO_PKG_REPOSITORY"))
            )
        }
    }
}

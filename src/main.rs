use std::collections::HashMap;

use base::errors::ConfigError;
use base::loader::FileConfigLoader;
use base::required_str;
use derive::EnvBuilder;

/// Example target struct
#[derive(Debug, EnvBuilder)]
pub struct Settings {
    pub name: String,
    pub server_env: String,
    pub version: u64,
}

fn main() -> Result<(), ConfigError> {
    let settings = SettingsBuilder::from_file(".env")?;
    println!("{:?}", settings);
    Ok(())
}

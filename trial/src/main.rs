use base::{errors::ConfigError, loader::FileConfigLoader};
use derive::EnvBuilder;
use std::collections::HashMap;

#[derive(EnvBuilder, Debug)]
pub struct Settings {
    pub env: String,
    pub port: Option<u16>,
}
fn main() -> Result<(), ConfigError> {
    let settings = SettingsBuilder::from_file(".example.env")?;

    println!("{:#?}", settings);

    Ok(())
}

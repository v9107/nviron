use std::collections::HashMap;

use base::errors::ConfigError;
use base::field::{Field, FieldBuilder};
use base::loader::{ConfigLoader, FileConfigLoader};
use base::required_str;

/// Example target struct
#[derive(Debug)]
pub struct Settings {
    pub name: String,
    pub server_env: String,
    pub version: u64,
}

#[derive(Debug, Default)]
pub struct SettingsBuilder<'a> {
    name: Field<'a, String>,
    server_env: Field<'a, String>,
    version: Field<'a, u64>,
}

impl<'a> SettingsBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Field::new("name", name.into());
        self
    }

    pub fn with_server_env<S: Into<String>>(mut self, server_env: S) -> Self {
        self.server_env = Field::new("server_env", server_env.into());
        self
    }

    pub fn with_version(mut self, version: u64) -> Self {
        self.version = Field::new("version", version);
        self
    }

    /// Return Result instead of panicking.
    pub fn build(self) -> Result<Settings, ConfigError> {
        Ok(Settings {
            name: self.name.value(),
            server_env: self.server_env.value(),
            version: self.version.value(),
        })
    }
}

impl ConfigLoader for SettingsBuilder<'_> {
    type Out = Settings;

    fn from_hash_map(map: HashMap<String, String>) -> Result<Self::Out, ConfigError> {
        let name = FieldBuilder::new("name")
            .with_value(required_str(&map, "name")?)
            .build::<String>()?;

        let server_env = FieldBuilder::new("server_env")
            .with_value(required_str(&map, "server_env").unwrap_or_else(|_| "local".to_lowercase()))
            .build::<String>()?;

        let version = FieldBuilder::new("version")
            .with_value(required_str(&map, "version")?)
            .build::<u64>()?;

        SettingsBuilder::new()
            .with_name(name.value())
            .with_server_env(server_env.value())
            .with_version(version.value())
            .build()
    }
}

fn main() -> Result<(), ConfigError> {
    let settings = SettingsBuilder::from_file(".env")?;
    println!("{:?}", settings);
    Ok(())
}

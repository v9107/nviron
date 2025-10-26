#![allow(dead_code, unused)]

use std::{collections::HashMap, str::FromStr};

mod errors;
mod reader;

use errors::ConfigError;

pub use crate::errors::ConfigError as Error;
pub use crate::reader::parse_env_contents;

/// Example target struct
#[derive(Debug)]
pub struct Settings {
    pub name: String,
    pub server_env: String,
    pub version: u64,
}

#[derive(Debug, Default)]
pub struct SettingsBuilder {
    name: Option<String>,
    server_env: Option<String>,
    version: Option<u64>,
}

impl SettingsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_server_env<S: Into<String>>(mut self, server_env: S) -> Self {
        self.server_env = Some(server_env.into());
        self
    }

    pub fn with_version(mut self, version: u64) -> Self {
        self.version = Some(version);
        self
    }

    /// Return Result instead of panicking.
    pub fn build(self) -> Result<Settings, ConfigError> {
        Ok(Settings {
            name: self
                .name
                .ok_or(ConfigError::MissingKey { key: "name".into() })?,
            server_env: self.server_env.unwrap_or_else(|| "local".into()),
            version: self.version.unwrap_or(0),
        })
    }
}

/* -------------------------
Helper parsing utilities
------------------------- */

/// Get a required string from the map (cloned)
pub fn required_str(map: &HashMap<String, String>, key: &str) -> Result<String, ConfigError> {
    map.get(key).cloned().ok_or(ConfigError::MissingKey {
        key: key.to_string(),
    })
}

/// Parse a required value using FromStr
pub fn required_parse<T>(map: &HashMap<String, String>, key: &str) -> Result<T, ConfigError>
where
    T: FromStr,
    <T as FromStr>::Err: ToString,
{
    let s = required_str(map, key)?;
    s.parse::<T>().map_err(|e| ConfigError::ParseError {
        key: key.to_string(),
        value: s,
        err: e.to_string(),
    })
}

/// Parse an optional value using FromStr
pub fn optional_parse<T>(map: &HashMap<String, String>, key: &str) -> Result<Option<T>, ConfigError>
where
    T: FromStr,
    <T as FromStr>::Err: ToString,
{
    match map.get(key) {
        None => Ok(None),
        Some(s) => Ok(Some(s.parse::<T>().map_err(|e| {
            ConfigError::ParseError {
                key: key.to_string(),
                value: s.clone(),
                err: e.to_string(),
            }
        })?)),
    }
}

/* -------------------------
Traits: loader + file loader
------------------------- */

/// Core loader trait — returns the canonical ConfigError.
/// This simpler signature is friendlier for derive macros.
pub trait ConfigLoader: Sized {
    type Out;
    fn from_hash_map(map: HashMap<String, String>) -> Result<Self::Out, ConfigError>;
}

/// Trait providing file-based loading with a default implementation
pub trait FileConfigLoader: ConfigLoader {
    fn from_file(path: &str) -> Result<Self::Out, ConfigError> {
        let contents = reader::read_contents(path).map_err(ConfigError::from)?;
        let map = reader::parse_env_contents(&contents);
        Self::from_hash_map(map)
    }
}

/// Blanket impl to give all ConfigLoader types the FileConfigLoader method.
impl<T: ConfigLoader> FileConfigLoader for T {}

/// Implement the loader for SettingsBuilder — this populates & validates fields.
impl ConfigLoader for SettingsBuilder {
    type Out = Settings;

    fn from_hash_map(map: HashMap<String, String>) -> Result<Self::Out, ConfigError> {
        let name = required_str(&map, "name")?;
        // server_env default handled by builder; treat as optional here
        let server_env = map
            .get("server_env")
            .cloned()
            .unwrap_or_else(|| "local".into());
        let version = required_parse::<u64>(&map, "version")?;

        SettingsBuilder::new()
            .with_name(name)
            .with_server_env(server_env)
            .with_version(version)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_generator_from_map() -> Result<(), Box<dyn Error>> {
        let mut map = HashMap::new();
        map.insert("name".into(), "venkatesh".into());
        // server_env is omitted -> default "local"
        map.insert("version".into(), "42".into());

        let settings = SettingsBuilder::from_hash_map(map)?;
        assert_eq!(settings.name, "venkatesh");
        assert_eq!(settings.server_env, "local");
        assert_eq!(settings.version, 42);
        Ok(())
    }

    #[test]
    fn test_generator_from_file() -> Result<(), Box<dyn Error>> {
        // if you have a `.env` fixture at repo root, the next line works:
        // let settings = SettingsBuilder::from_file(".env")?;
        // For CI-less test demonstration we build a temporary string:
        let contents = "name=venkatesh\nserver_env=local\nversion=7\n";
        let map = reader::parse_env_contents(contents);
        let settings = SettingsBuilder::from_hash_map(map)?;
        assert_eq!(settings.name, "venkatesh");
        assert_eq!(settings.server_env, "local");
        assert_eq!(settings.version, 7_u64);
        Ok(())
    }
}

use std::{collections::HashMap, str::FromStr};

pub mod errors;
pub mod field;
pub mod loader;
pub mod parser;
pub mod reader;

use errors::ConfigError;

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

/// Implement the loader for SettingsBuilder — this populates & validates fields.

#[cfg(test)]
mod tests {
    use super::*;

    use crate::errors::ConfigError;
    use crate::field::{Field, FieldBuilder};
    use crate::loader::ConfigLoader;
    use crate::reader;

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

            // server_env default handled by builder; treat as optional here
            let server_env = FieldBuilder::new("server_env")
                .with_value(
                    required_str(&map, "server_env").unwrap_or_else(|_| "local".to_lowercase()),
                )
                .build::<String>()?;

            println!("{:?}", server_env);

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

    #[test]
    fn test_generator_from_map() -> Result<(), ConfigError> {
        let mut map = HashMap::new();
        map.insert("name".into(), "test".into());
        // server_env is omitted -> default "local"
        map.insert("version".into(), "42".into());

        let settings = SettingsBuilder::from_hash_map(map)?;
        assert_eq!(settings.name, "test");
        assert_eq!(settings.server_env, "local");
        assert_eq!(settings.version, 42_u64);
        Ok(())
    }
}

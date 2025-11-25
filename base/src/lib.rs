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
        pub last_name: Option<String>,
    }

    #[derive(Debug, Default)]
    pub struct SettingsBuilder {
        name: Field,
        server_env: Field,
        last_name: Field,
    }

    impl SettingsBuilder {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_name(mut self, name_bldr: FieldBuilder) -> Result<Self, ConfigError> {
            self.name = name_bldr.build()?;
            Ok(self)
        }

        pub fn with_server_env(
            mut self,
            server_env_bldr: FieldBuilder,
        ) -> Result<Self, ConfigError> {
            self.server_env = server_env_bldr.build()?;
            Ok(self)
        }

        pub fn with_last_name(mut self, last_name_bldr: FieldBuilder) -> Result<Self, ConfigError> {
            self.last_name = last_name_bldr.build()?;
            Ok(self)
        }

        pub fn build(self) -> Result<Settings, ConfigError> {
            let name = self
                .name
                .value()?
                .ok_or(ConfigError::missing_key_err("name"))?;

            let server_env = self
                .server_env
                .value()?
                .ok_or(ConfigError::missing_key_err("server_env"))?;

            let last_name = self.last_name.value()?;

            Ok(Settings {
                name,
                server_env,
                last_name,
            })
        }
    }

    impl ConfigLoader for SettingsBuilder {
        type Out = Settings;

        fn from_hash_map(map: HashMap<String, String>) -> Result<Self::Out, ConfigError> {
            let name = FieldBuilder::new("name").with_value(required_str(&map, "name").ok());

            let server_env = FieldBuilder::new("server_env").with_value(
                required_str(&map, "server_env")
                    .or_else(|_| Ok::<String, ConfigError>("local".to_string()))
                    .ok(),
            );

            let last_name = FieldBuilder::new("last_name")
                .with_optional(true)
                .with_value(optional_parse(&map, "last_name")?);

            SettingsBuilder::new()
                .with_name(name)?
                .with_last_name(last_name)?
                .with_server_env(server_env)?
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
        assert_eq!(settings.last_name, None);
        Ok(())
    }
}

use std::{collections::HashMap, str::FromStr};

pub mod errors;
pub mod field;
pub mod loader;
pub mod parser;
pub mod reader;
pub mod v2;

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

use std::fmt;

/// Centralized error type for the config loader
#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    MissingKey {
        key: String,
    },
    ParseError {
        key: String,
        value: String,
        err: String,
    },
    LoadingError {
        path: String,
        err: String,
    },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Io(err) => write!(f, "I/O error: {}", err),
            ConfigError::MissingKey { key } => write!(f, "Missing required key '{}'", key),
            ConfigError::ParseError { key, value, err } => {
                write!(
                    f,
                    "Failed to parse key '{}': value '{}' ({})",
                    key, value, err
                )
            }
            ConfigError::LoadingError { path, err } => {
                write!(f, "Failed to load file '{}': {}", path, err)
            }
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError::Io(err)
    }
}

/// Convenience constructor for parse errors (optional)
impl ConfigError {
    #[allow(dead_code)]
    pub fn parse_err<T: ToString>(key: &str, value: &str, e: T) -> Self {
        ConfigError::ParseError {
            key: key.to_string(),
            value: value.to_string(),
            err: e.to_string(),
        }
    }
}

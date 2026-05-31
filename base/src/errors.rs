use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ConfigError {
    #[error("Missing key {key:?} or make it optional")]
    MissingKey { key: String },
    #[error("Parsing failed for {key:?} due to {err:?}")]
    ParseError {
        key: String,
        value: String,
        err: String,
    },
    #[error("Failed to load the file from {path:?} due to {err:?}")]
    LoadingError { path: String, err: String },
}

impl ConfigError {
    pub fn parse_err(key: &str, value: impl ToString, e: impl ToString) -> Self {
        ConfigError::ParseError {
            key: key.to_string(),
            value: value.to_string(),
            err: e.to_string(),
        }
    }

    pub fn missing_key_err(key: impl ToString) -> Self {
        ConfigError::MissingKey {
            key: key.to_string(),
        }
    }

    pub fn loading_err(path: impl ToString, err: impl ToString) -> Self {
        Self::LoadingError {
            path: path.to_string(),
            err: err.to_string(),
        }
    }
}

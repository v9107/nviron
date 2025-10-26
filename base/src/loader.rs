use crate::errors::ConfigError;
use crate::{parser, reader};
use std::collections::HashMap;

/// Core loader trait — returns the canonical ConfigError.
pub trait ConfigLoader: Sized {
    type Out;
    fn from_hash_map(map: HashMap<String, String>) -> Result<Self::Out, ConfigError>;
}

/// Trait providing file-based loading with a default implementation
pub trait FileConfigLoader: ConfigLoader {
    fn from_file(path: &str) -> Result<Self::Out, ConfigError> {
        let contents = reader::read_contents(path).map_err(ConfigError::from)?;
        let map = parser::parse_env_contents(&contents);
        Self::from_hash_map(map)
    }
}

/// Blanket impl to give all ConfigLoader types the FileConfigLoader method.
impl<T: ConfigLoader> FileConfigLoader for T {}

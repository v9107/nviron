use crate::errors::ConfigError;
use crate::reader;
use std::collections::HashMap;

/// Core loader trait — returns the canonical ConfigError.
pub trait ConfigLoader: Sized {
    type Out;
    fn from_hash_map(map: HashMap<String, String>) -> Result<Self::Out, ConfigError>;
}

/// Trait providing file-based loading with a default implementation
pub trait FileConfigLoader: ConfigLoader {
    fn from_file(path: &str) -> Result<Self::Out, ConfigError> {
        let contents =
            reader::read_contents(path).map_err(|err| ConfigError::loading_err(path, err))?;
        let map = load_env_contents(&contents);
        Self::from_hash_map(map)
    }
}

/// Blanket impl to give all ConfigLoader types the FileConfigLoader method.
impl<T: ConfigLoader> FileConfigLoader for T {}

/// Load the `.env` file contents into a HashMap<String,String>
/// - Trim whitespace
/// - Skip blank lines and comment lines starting with `#`
/// - Split on the first `=` only
/// - Strip surrounding quotes (single or double) from values
fn load_env_contents(contents: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();

    for raw_line in contents.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // splitn on the first '='
        let mut parts = line.splitn(2, '=');
        let key_part = parts.next().map(str::trim);
        let val_part = parts.next().map(str::trim);

        if let (Some(k), Some(v)) = (key_part, val_part) {
            if k.is_empty() {
                // skip invalid
                continue;
            }
            let mut value = v.to_string();
            // strip surrounding double or single quotes if present
            if (value.starts_with('"') && value.ends_with('"'))
                || (value.starts_with('\'') && value.ends_with('\''))
            {
                if value.len() >= 2 {
                    value = value[1..value.len() - 1].to_string();
                }
            }
            map.insert(k.to_string(), value);
        } else {
            // skip malformed lines without '='
            continue;
        }
    }

    map
}

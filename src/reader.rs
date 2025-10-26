use std::collections::HashMap;
use std::io;

/// Read file contents (thin wrapper)
pub fn read_contents(path: &str) -> io::Result<String> {
    std::fs::read_to_string(path)
}

/// Parse `.env` file contents into a HashMap<String,String>
/// - Trim whitespace
/// - Skip blank lines and comment lines starting with `#`
/// - Split on the first `=` only
/// - Strip surrounding quotes (single or double) from values
pub fn parse_env_contents(contents: &str) -> HashMap<String, String> {
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

use crate::errors::ConfigError;
use std::str::FromStr;

pub fn parse_to_result<T>(key: String, value: Option<String>) -> Result<T, ConfigError>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let v = value.ok_or(ConfigError::missing_key_err(key.to_owned()))?;

    let v = v
        .parse()
        .map_err(|err| ConfigError::parse_err(key.as_str(), v, err))?;

    Ok(v)
}

pub fn parse_optional_result<T>(
    key: String,
    value: Option<String>,
) -> Result<Option<T>, ConfigError>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    if value.is_none() {
        return Ok(None);
    };

    match parse_to_result(key.to_owned(), value) {
        Ok(v) => Ok(Some(v)),
        Err(e @ ConfigError::ParseError { .. }) => Err(e),
        Err(_) => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_required_parse() -> Result<(), ConfigError> {
        let result: String = parse_to_result("key".to_string(), Some("value".to_string()))?;
        assert_eq!(result, "value".to_string());
        Ok(())
    }

    #[test]
    fn test_optional_parse() -> Result<(), ConfigError> {
        let result: Option<String> = parse_optional_result("key".to_string(), None)?;
        assert!(result.is_none());
        Ok(())
    }
}

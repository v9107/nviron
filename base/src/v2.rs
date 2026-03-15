use crate::{errors::ConfigError, parser::parse_env_contents, reader::read_contents};
use std::{collections::HashMap, marker::PhantomData, str::FromStr};

/// Version 2
/// 1. Loader (Loads the config fiels as HashMap<String, String>)
/// 2. Parser
///     a. Extracts all the fields required build the struct
///     b. Convert the value and Returns the updated HashMap<String, T>(Parses each fields to its corresponding value)
/// 4. Builder (Takes the parser and builds the struct)

pub fn hashmap_from_file(path: &'_ str) -> Result<HashMap<String, String>, ConfigError> {
    let contents = read_contents(path).map_err(|err| ConfigError::loading_err(path, err))?;
    Ok(parse_env_contents(&contents))
}

pub struct ParsingInfo<T: FromStr> {
    key: String,
    _t: PhantomData<T>,
}

impl<T: FromStr> ParsingInfo<T> {
    pub fn new(k: impl ToString) -> ParsingInfo<T> {
        ParsingInfo {
            key: k.to_string(),
            _t: PhantomData,
        }
    }
}

pub fn parse_v2<T>(hp: &HashMap<String, String>, info: ParsingInfo<T>) -> Result<T, ConfigError>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let v = hp.get(&info.key);

    let v = v.ok_or(ConfigError::missing_key_err(info.key.to_owned()))?;
    let v = v
        .parse()
        .map_err(|err| ConfigError::parse_err(info.key.as_str(), v, err));
    v
}

pub fn parse_option<T>(
    hp: &HashMap<String, String>,
    info: ParsingInfo<T>,
) -> Result<Option<T>, ConfigError>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let v = hp.get(&info.key).map(|v| v.to_owned());

    let v = v.map_or(Ok(None), |_| {
        let r = parse_v2(hp, info);
        let r = r.map_or(None, Some);
        Ok(r)
    });

    v
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        errors::ConfigError,
        v2::{ParsingInfo, parse_option, parse_v2},
    };

    #[test]
    fn test_parse_v2() -> Result<(), ConfigError> {
        let mut map = HashMap::<String, String>::new();
        map.insert("name".into(), "test".into());
        // server_env is omitted -> default "local"
        map.insert("version".into(), "42".into());

        let name_info = ParsingInfo::<String>::new("name");
        let version_info = ParsingInfo::<u32>::new("version");

        let name = parse_v2(&map, name_info)?;
        let version = parse_v2(&map, version_info)?;

        assert_eq!(name, "test".to_string());
        assert_eq!(version, 42);

        Ok(())
    }

    #[test]
    fn test_option_parse_some() -> Result<(), ConfigError> {
        let mut map = HashMap::<String, String>::new();
        map.insert("version".into(), "42".into());

        let version_info = ParsingInfo::<u32>::new("version");

        let version = parse_option(&map, version_info)?;

        assert_eq!(version, Some(42));

        Ok(())
    }

    #[test]
    fn test_option_parse_none() -> Result<(), ConfigError> {
        let mut map = HashMap::<String, String>::new();
        map.insert("name".into(), "test".into());

        let version_info = ParsingInfo::<u32>::new("version");

        let version = parse_option(&map, version_info)?;

        assert_eq!(version, None);

        Ok(())
    }
}

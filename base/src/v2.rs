use crate::{
    errors::ConfigError,
    optional_parse,
    parser::{parse, parse_env_contents},
    parser_helper::{parse_optional_result, parse_to_result},
    reader::read_contents,
};
use core::fmt;
use std::{collections::HashMap, marker::PhantomData, str::FromStr};

mod builder;

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

#[derive(Debug)]
pub struct FieldInfo<T: FromStr> {
    key: String,
    optional: bool,
    value: Option<String>,
    _t: PhantomData<T>,
}

impl<T: FromStr> FieldInfo<T> {
    pub fn new(k: impl ToString) -> FieldInfo<T> {
        FieldInfo {
            key: k.to_string(),
            optional: false,
            value: None,
            _t: PhantomData,
        }
    }

    pub fn with_value(self, v: Option<String>) -> Self {
        FieldInfo { value: v, ..self }
    }
}
struct Optional<T>(pub Option<T>);

trait Reader<T> {
    fn read(self) -> T;
}

impl<T> Reader<T> for T {
    fn read(self) -> T {
        self
    }
}

impl<T> Reader<Option<T>> for Optional<T> {
    fn read(self) -> Option<T> {
        self.0
    }
}

pub trait ParserV2: Sized {
    fn parsev2(key: String, v: Option<String>) -> Result<impl Reader<Self>, ConfigError>;
}

impl<T> ParserV2 for T
where
    T: FromStr,
    T::Err: fmt::Display,
{
    fn parsev2(key: String, v: Option<String>) -> Result<impl Reader<Self>, ConfigError> {
        parse_to_result::<T>(key, v)
    }
}

impl<T> ParserV2 for Optional<T>
where
    T: FromStr,
    T::Err: fmt::Display,
{
    fn parsev2(key: String, v: Option<String>) -> Result<impl Reader<Self>, ConfigError> {
        let r = parse_optional_result::<T>(key, v)?;
        Ok(Optional(r))
    }
}

//impl<T> ParserV2 for Option<T>
//{
//    fn parsev2(key: String, v: Option<String>) -> Result<impl Reader<Self>, ConfigError> {
//        parse_optional_result(key, v)
//    }
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reader() -> Result<(), ConfigError> {
        let hello = "hello".read();

        assert_eq!(hello, "hello");
        Ok(())
    }

    #[test]
    fn test_optional() -> Result<(), ConfigError> {
        let info = FieldInfo::<String>::new("key").with_value(None);

        let r = Optional::<String>::parsev2(info.key, info.value)?;
        let r = r.read();
        assert_eq!(r, None);

        Ok(())
    }

    #[test]
    fn test_field_parser() -> Result<(), ConfigError> {
        let v: String = String::parsev2("key".to_string(), Some("value".to_string()))?.read();
        assert_eq!(v, "value".to_string());

        Ok(())
    }
}

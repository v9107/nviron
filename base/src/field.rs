use std::str::FromStr;

use crate::{errors::ConfigError, parser::Parser};

#[derive(Debug, Default)]
pub struct Field<'a, T>
where
    T: Clone,
{
    key: &'a str,
    value: T,
}

impl<'a, T> Field<'a, T>
where
    T: Clone,
{
    pub fn new(key: impl Into<&'a str>, value: T) -> Self {
        Self {
            key: key.into(),
            value,
        }
    }

    pub fn value(&self) -> T {
        self.value.clone()
    }
}

pub struct FieldBuilder<'a> {
    key: &'a str,
    value: Option<String>,
}

impl<'a, U> Parser<FieldBuilder<'a>, U> for FieldBuilder<'a>
where
    U: Clone + FromStr,
    <U as FromStr>::Err: ToString,
{
    type Out = Result<U, ConfigError>;

    fn parse(to_parse: FieldBuilder<'a>) -> Self::Out {
        let FieldBuilder { key, value } = to_parse;

        let v = value.ok_or_else(|| ConfigError::ParseError {
            key: key.to_string(),
            value: "None".to_string(),
            err: "cannot parse `None` type".to_string(),
        })?;

        v.parse::<U>().map_err(|err| ConfigError::ParseError {
            key: key.to_lowercase(),
            value: v,
            err: err.to_string(),
        })
    }
}

impl<'a> FieldBuilder<'a> {
    pub fn new(key: impl Into<&'a str>) -> Self {
        Self {
            key: key.into(),
            value: None,
        }
    }

    pub fn with_value(mut self, value: String) -> Self {
        self.value = Some(value);
        self
    }

    pub fn build<T>(self) -> Result<Field<'a, T>, ConfigError>
    where
        T: Clone + FromStr,
        <T as FromStr>::Err: ToString,
    {
        Ok(Field {
            key: self.key,
            value: Self::parse(self)?,
        })
    }
}

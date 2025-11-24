use std::str::FromStr;

use crate::errors::ConfigError;
use crate::parser;

#[derive(Debug, Default)]
pub struct Field {
    key: String,
    value: Option<String>,
    optional: bool,
}

impl Field {
    pub fn new(key: impl ToString, value: Option<String>) -> Self {
        Self {
            key: key.to_string(),
            value,
            optional: false,
        }
    }

    pub fn with_optional(mut self, op: bool) -> Self {
        self.optional = op;
        self
    }

    pub fn is_optional(&self) -> bool {
        self.optional
    }

    pub fn value<T>(self) -> Result<Option<T>, ConfigError>
    where
        T: Clone + std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Display,
    {
        if self.is_optional() && self.value.is_none() {
            return Ok(None);
        }

        match self.value {
            Some(v) => parser::parse(v).map(Some),
            None => Err(ConfigError::missing_key_err(self.key)),
        }
    }
}

pub struct FieldBuilder {
    key: String,
    value: Option<String>,
    optional: bool,
}

impl FieldBuilder {
    pub fn new(key: impl ToString) -> Self {
        Self {
            key: key.to_string(),
            value: None,
            optional: false,
        }
    }

    pub fn with_value(mut self, value: Option<String>) -> Self {
        self.value = value;
        self
    }

    pub fn with_optional(mut self, op: bool) -> Self {
        self.value = None;
        self.optional = op;
        self
    }

    pub fn build(self) -> Result<Field, ConfigError> {
        if self.value.is_some() || self.optional {
            return Ok(Field {
                key: self.key,
                value: self.value,
                optional: self.optional,
            });
        }

        Err(ConfigError::missing_key_err(self.key))
    }
}

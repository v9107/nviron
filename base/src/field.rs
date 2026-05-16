use crate::errors::ConfigError;
use crate::parser::ParseValue;

#[derive(Debug, Default)]
pub struct Field {
    key: String,
    value: Option<String>,
}

impl Field {
    pub fn new(key: impl ToString, value: Option<String>) -> Self {
        Self {
            key: key.to_string(),
            value,
        }
    }

    pub fn parse<T>(self) -> Result<T, ConfigError>
    where
        T: ParseValue,
    {
        T::parse_value(self.key, self.value)
    }
}

pub struct FieldBuilder {
    key: String,
    value: Option<String>,
}

impl FieldBuilder {
    pub fn new(key: impl ToString) -> Self {
        Self {
            key: key.to_string(),
            value: None,
        }
    }

    pub fn with_value(mut self, value: Option<String>) -> Self {
        self.value = value;
        self
    }

    pub fn build(self) -> Field {
        Field {
            key: self.key,
            value: self.value,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_field_parsing_for_required() -> Result<(), ConfigError> {
        let field = Field::new("key", Some("32".to_string()));
        let res = field.parse::<u16>()?;
        assert_eq!(32 as u16, res);
        Ok(())
    }

    #[test]
    fn test_field_parsing_for_option_field() -> Result<(), ConfigError> {
        let field = Field::new("key", Some("32".to_string()));
        let res = field.parse::<Option<u16>>()?;
        assert_eq!(Some(32 as u16), res);
        Ok(())
    }

    #[test]
    fn test_failed_parsing_for_required_field() -> Result<(), ConfigError> {
        let field = Field::new("key", Some("hello".to_string()));
        let res = field.parse::<u16>();
        assert!(res.is_err());
        Ok(())
    }

    #[test]
    fn test_failed_parsing_for_optional_field() -> Result<(), ConfigError> {
        let field = Field::new("key", Some("hello".to_string()));
        let res = field.parse::<Option<u16>>();
        assert!(res.is_err());
        Ok(())
    }

    #[test]
    fn test_missing_value_for_parsing_optional_field() -> Result<(), ConfigError> {
        let field = Field::new("key", None);
        let res = field.parse::<Option<u16>>()?;
        assert!(res.is_none());
        Ok(())
    }

    #[test]
    fn test_missing_value_for_parsing_required_field() -> Result<(), ConfigError> {
        let field = Field::new("key", None);
        let res = field.parse::<u16>();
        assert!(res.is_err());
        Ok(())
    }
}

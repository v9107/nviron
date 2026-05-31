use crate::{errors::ConfigError, parser_helper::parse_option};

pub trait ParseValue: Sized {
    fn parse_value(key: String, value: Option<String>) -> Result<Self, ConfigError>;
}

impl<T> ParseValue for Option<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    fn parse_value(key: String, value: Option<String>) -> Result<Self, ConfigError> {
        parse_option(key, value)
    }
}

#[macro_export]
macro_rules! impl_parse_value {
    ($($t:ty),*) => {
        $(
            impl ParseValue for $t {
                fn parse_value(key: String, value: Option<String>) -> Result<Self, ConfigError> {
                    $crate::parser_helper::parse_required::<$t>(key, value)
                }
            }
        )*
    };
}

impl_parse_value!(u8, u16, u32, u64, i8, i16, i32, i64, f32, f64, bool, String);

#[cfg(test)]
mod tests {
    use crate::{errors::ConfigError, parser::ParseValue};

    #[test]
    fn test_parsing_optional_type() -> Result<(), ConfigError> {
        let val: Option<String> = Some("32".to_string());
        let key: String = "key".to_string();

        let re: Result<Option<u32>, ConfigError> = Option::<u32>::parse_value(key, val);

        assert!(re.is_ok());
        Ok(())
    }

    #[test]
    fn test_parsing_required_type() -> Result<(), ConfigError> {
        let val: Option<String> = Some("32".to_string());
        let key: String = "key".to_string();

        let re: Result<u32, ConfigError> = u32::parse_value(key, val);

        assert!(re.is_ok());
        Ok(())
    }

    #[test]
    fn test_parsing_faliure_for_required_type() -> Result<(), ConfigError> {
        let val: Option<String> = Some("hello".to_string());
        let key: String = "key".to_string();

        let re: Result<u32, ConfigError> = u32::parse_value(key, val);

        assert!(re.is_err());
        Ok(())
    }

    #[test]
    fn test_parsing_faliure_for_option_type() -> Result<(), ConfigError> {
        let val: Option<String> = Some("hello".to_string());
        let key: String = "key".to_string();
        let re: Result<Option<u32>, ConfigError> =
            <Option<u32> as ParseValue>::parse_value(key, val);

        assert!(re.is_err());
        Ok(())
    }
    #[test]
    fn test_parsing_and_extraction_of_required_type() -> Result<(), ConfigError> {
        let val: Option<String> = Some("32".to_string());
        let key: String = "key".to_string();

        let re: u32 = u32::parse_value(key, val)?;

        assert_eq!(32, re);
        Ok(())
    }

    #[test]
    fn test_parser_integration() -> Result<(), ConfigError> {
        let k = "key".to_string();
        let v = Some("32".to_string());
        let res: u32 = u32::parse_value(k, v)?;

        assert_eq!(32, res);

        Ok(())
    }

    #[test]
    fn test_parser_integration_for_option_type() -> Result<(), ConfigError> {
        let k = "key".to_string();
        let v = Some("32".to_string());
        let res: Option<u32> = Option::<u32>::parse_value(k, v)?;

        assert_eq!(Some(32), res);

        Ok(())
    }
}

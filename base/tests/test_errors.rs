use base::errors::ConfigError;


#[test]
fn test_config_parsing_error() -> Result<(), Box<dyn std::error::Error>> {
    let e = ConfigError::parse_err("key", "value", "Failed to parse");
    let expected = "Parsing failed for \"key\" due to \"Failed to parse\"".to_string();
    assert_eq!(expected, e.to_string());
    Ok(())
}

#[test]
fn test_config_missing_key_error() -> Result<(), Box<dyn std::error::Error>> {
    let e = ConfigError::missing_key_err("key");
    let expected = "Missing key \"key\" or make it optional".to_string();
    assert_eq!(expected, e.to_string());
    Ok(())
}

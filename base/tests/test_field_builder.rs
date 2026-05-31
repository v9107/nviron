use base::field::FieldBuilder;
use base::errors::ConfigError;


#[test]
fn test_create_field_builder() -> Result<(), ConfigError> {
    let bldr = FieldBuilder::new("random_key")
        .with_value(Some("hello".to_string()));

    let field = bldr.build();

    assert_eq!("hello".to_string(), field.parse::<String>()?);
    Ok(())
}

use base::errors::ConfigError;
use base::field::FieldBuilder;
use base::loader::FileConfigLoader;
use derive::FromEnv;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(FromEnv)]
pub struct BasicExample {
    pub env: String,
    pub port: u32,
}

#[test]
fn test_building_basic_struct_from_env() -> Result<(), ConfigError> {
    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    file_path.push("./resources/basic.example.env");

    let file_path = file_path
        .to_str()
        .expect("Failed while creating path for resources");

    println!("{:?}", file_path);

    let b = BasicExampleBuilder::from_file(file_path)?;
    let expected_env = "local".to_string();
    let expected_port: u32 = 8080;
    assert_eq!(expected_env, b.env);
    assert_eq!(expected_port, b.port);
    Ok(())
}

#[test]
fn test_e2e_field_builder() -> Result<(), ConfigError> {
    let bldr = FieldBuilder::new("random_key").with_value(Some("hello".to_string()));

    let field = bldr.build();

    assert_eq!("hello".to_string(), field.parse::<String>()?);
    Ok(())
}

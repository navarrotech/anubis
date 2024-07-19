// Copyright © 2024 Navarrotech

// Custom modules
use crate::schema::AnubisSchema;

pub fn setup_anubis_schema(schema: &AnubisSchema) {
    let schema_path = schema.install_directory.clone().join("Anubis.yaml");
    let schema_content = create_anubis_schema(schema);

    // Write the schema to the file
    std::fs::write(schema_path, schema_content).expect("Unable to write Anubis.yaml file");
}

pub fn create_anubis_schema(schema: &AnubisSchema) -> String {
    let copyright = if schema.copyright_header_formatted.is_empty() {
        schema.copyright_header_formatted.clone()
    } else {
        format!("# {}", schema.copyright_header_formatted)
    };

    format!(
        "{copyright_formatted}

# Anubis.yaml
# 
# This file is an Anubis relic, meaning it was only auto-generated during initialization.
# You may safely modify this file as much as you want, and Anubis will not touch this file again.
# Anubis will use this file to store all details regarding your project.
# 
# You can use the command `anubis validate` to ensure this file is valid.

project:
  name: '{project_name}'
  version: '{project_version}'
  copyright: '{copyright_unformatted}'
  description: '{description}'
",
        project_name = schema.project_name,
        project_version = schema.version,
        description = schema.description,
        copyright_unformatted = schema.copyright_header,
        copyright_formatted = copyright
    )
}

#[cfg(test)]
mod check_anubis_schema {
    use super::*;
    use crate::schema::AnubisSchema;
    use serde_yaml;
    use serde_yaml::Error;
    use tempfile::tempdir;

    fn is_valid_yaml(yaml_str: &str) -> Result<(), Error> {
        serde_yaml::from_str::<serde_yaml::Value>(yaml_str).map(|_| ())
    }

    fn mock_schema() -> AnubisSchema {
        let mut test_schema = AnubisSchema::default();
        test_schema.project_name = "Anubis Test".to_string();
        test_schema.copyright_header = String::from("// Copyright © {YYYY} Navarrotech");
        test_schema.copyright_header_formatted = String::from("// Copyright © 2024 Navarrotech");

        test_schema
    }

    #[test]
    fn ensure_anubis_schema_yaml_is_valid() {
        let test_schema = mock_schema();

        let content = create_anubis_schema(&test_schema);

        assert!(is_valid_yaml(content.as_str()).is_ok());
    }

    #[test]
    fn ensure_anubis_schema_writes_the_file() {
        let temp_directory = tempdir().unwrap().into_path();

        let mut test_schema = mock_schema();
        test_schema.install_directory = temp_directory.clone();

        setup_anubis_schema(&test_schema);

        let file_path = temp_directory.join("Anubis.yaml");
        assert!(file_path.exists());

        let content = std::fs::read_to_string(file_path).unwrap();
        assert!(is_valid_yaml(content.as_str()).is_ok());
    }
}

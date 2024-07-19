// Copyright © 2024 Navarrotech

// Lib
use std::fs;
use std::io;

// Custom modules
use crate::schema::AnubisSchema;

pub fn setup_anubis_schema(schema: &AnubisSchema) -> io::Result<()> {
    let schema_path = schema.base_path.join("Anubis.yaml");
    let schema_content = create_anubis_schema(schema);
}

pub fn create_anubis_schema(schema: &AnubisSchema) -> String {
    format!("{copyright_formatted}

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
")
}

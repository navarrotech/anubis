// Copyright © 2024 Navarrotech

// Lib
extern crate yaml_rust;
use chrono::Datelike;
use std::collections::HashMap;
use yaml_rust::{Yaml, YamlLoader};

// Custom modules
use crate::schema::AnubisSchema;

#[derive(Debug)]
struct ProjectSchema {
    name: Option<String>,
    version: Option<String>,
    copyright_header: Option<String>,
    description: Option<String>,
}

pub fn parse_schema_yaml() -> AnubisSchema {
    println!("Parsing schema.yaml...");

    // Read schema.yaml file
    let yaml_content =
        std::fs::read_to_string("./examples/Anubis.yaml").expect("Could not read schema.yaml file");

    // Parse schema.yaml file
    let docs = YamlLoader::load_from_str(&yaml_content).unwrap();
    // let schema: YamlSchema = serde_yaml::from_str(&yaml_content).expect("Could not parse schema.yaml file");

    let doc = &docs[0];

    if doc["project"].is_badvalue() {
        panic!("Invalid schema.yaml file. 'project' is required.");
    }

    if doc["project"]["name"].is_badvalue() {
        panic!("Invalid schema.yaml file. 'project.name' is required.");
    }

    let project = parse_project_schema(&doc["project"]);

    println!("Schema: {:?}", project);
    println!("Schema parsed successfully!");

    let now = chrono::Utc::now();
    let year = now.year();

    let copyright_header = match project.copyright_header {
        Some(ref s) => s.to_string(),
        None => String::from(""),
    };

    let copyright_header_formatted = match project.copyright_header {
        Some(ref s) => s.to_string().replace("{YYYY}", &year.to_string()),
        None => String::from(""),
    };

    let description = match project.description {
        Some(ref s) => s.to_string(),
        None => String::from(""),
    };

    let version = match project.version {
        Some(ref s) => s.to_string(),
        None => String::from(""),
    };

    AnubisSchema {
        project_name: project.name.unwrap(),
        version,
        description,
        copyright_header,
        copyright_header_formatted,
        install_directory: std::env::current_dir().unwrap(),
        ..Default::default()
    }
}

fn parse_project_schema(yaml: &Yaml) -> ProjectSchema {
    ProjectSchema {
        name: yaml["name"].as_str().map(|s| s.to_string()),
        version: yaml["version"].as_str().map(|s| s.to_string()),
        copyright_header: yaml["copyright_header"].as_str().map(|s| s.to_string()),
        description: yaml["description"].as_str().map(|s| s.to_string()),
    }
}

// Copyright © 2024 Navarrotech

// Lib
extern crate yaml_rust;
use chrono::Datelike;
use yaml_rust::{Yaml, YamlLoader};

// Custom modules
use crate::models::{FormatChoice, ModelFields, ModelKind, Models, UseOption};
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

    let models = if doc["models"].is_badvalue() {
        Vec::new()
    } else {
        parse_models(&doc["models"])
    };

    AnubisSchema {
        project_name: project.name.unwrap(),
        version,
        description,
        copyright_header,
        copyright_header_formatted,
        install_directory: std::env::current_dir().unwrap(),
        models,
        ..Default::default()
    }
}

fn parse_models(yaml: &Yaml) -> Vec<Models> {
    let mut models = Vec::new();

    // For each model...
    for (key, value) in yaml.as_hash().unwrap() {
        let mut model = Models::default();

        // If it has a 'fields' key...
        if !value["fields"].is_badvalue() {
            let mut model_fields = ModelFields::default();
            let fields_array = value["fields"].as_vec().unwrap();

            for field in fields_array {
                let fields = field.as_hash().unwrap();

                for (field_key, field_value) in fields {
                    let field_name = field_key.as_str().unwrap();
                    let field_value = field_value.as_str().unwrap_or("");

                    match field_name {
                        // Core fields, required
                        "name" => model_fields.name = field_value.to_string(),
                        "kind" => {
                            model_fields.kind = match field_value {
                                "string" => ModelKind::String,
                                "number" => ModelKind::Number,
                                "float" => ModelKind::Float,
                                "boolean" => ModelKind::Boolean,
                                "datetime" => ModelKind::DateTime,
                                "date" => ModelKind::DateTime,
                                _ => ModelKind::String,
                            }
                        }

                        // Core fields, optional
                        "default" => model_fields.default = Some(field_value.to_string()),

                        // Boolean fields (default false)
                        "primary_key" => model_fields.primary_key = true,
                        "required" => model_fields.required = true,
                        "encrypt" => model_fields.encrypt = true,
                        "replicate" => model_fields.replicate = true,
                        "unique" => model_fields.unique = true,

                        // Enums
                        "use" => {
                            model_fields.use_method = match field_value {
                                "uuid" => Some(UseOption::Uuid),
                                "unique" => Some(UseOption::Unique),
                                "owner" => Some(UseOption::OwnerLink),
                                "created_at" => Some(UseOption::CreatedAt),
                                "updated_at" => Some(UseOption::UpdatedAt),
                                _ => None,
                            }
                        }
                        "format" => {
                            model_fields.format = match field_value {
                                "email" => Some(FormatChoice::Email),
                                "phone" => Some(FormatChoice::Phone),
                                "password" => Some(FormatChoice::Password),
                                "secret" => Some(FormatChoice::Secret),
                                _ => None,
                            }
                        }

                        // Number fields
                        "min" | "minimum" => {
                            model_fields.minimum = field_value.to_string().parse::<u32>().ok();
                        }
                        "max" | "maximum" => {
                            model_fields.maximum = field_value.to_string().parse::<u32>().ok();
                        }

                        // Misc
                        "replace_all" => {
                            model_fields.replace_all = Some(field_value.parse().unwrap())
                        }
                        "match" => model_fields.use_match = Some(field_value.to_string()),
                        "on_unknown" => model_fields.on_unknown = Some(field_value.to_string()),
                        _ => (),
                    }
                }
            }

            model.fields.push(model_fields);
        }

        models.push(model);
    }

    models
}

fn parse_project_schema(yaml: &Yaml) -> ProjectSchema {
    ProjectSchema {
        name: yaml["name"].as_str().map(|s| s.to_string()),
        version: yaml["version"].as_str().map(|s| s.to_string()),
        copyright_header: yaml["copyright_header"].as_str().map(|s| s.to_string()),
        description: yaml["description"].as_str().map(|s| s.to_string()),
    }
}

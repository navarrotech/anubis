// Copyright © 2024 Navarrotech

pub enum ModelKind {
    String,
    Number,
    Boolean,
}

pub enum UseOption {
    Uuid,
    Unique,
    CreatedAt,
    UpdatedAt,
}

pub enum FormatChoice {
    Email,
    Phone,
    Password,
    Secret,
}

pub struct ModelFields {
    // Core fields, required
    pub name: String,
    pub kind: ModelKind,

    // Core fields, optional
    pub default: Option<String>,

    // Boolean fields (default false)
    pub primary_key: bool,
    pub required: bool,
    pub encrypt: bool,
    pub replicate: bool,
    pub unique: bool,

    // Enums
    pub r#use: Option<UseOption>,
    pub format: Option<FormatChoice>,

    // Number fields
    pub minimum: Option<u32>,
    pub maximum: Option<u32>,

    // Misc
    pub replace_all: Option<String>,
    pub on_unknown: Option<String>,
    pub r#match: Option<String>,
    pub r#enum: Option<Vec<String>>,
    pub links: Option<String>,
}

impl Default for ModelFields {
    fn default() -> Self {
        ModelFields {
            name: String::new(),
            kind: ModelKind::String,
            r#use: None,
            primary_key: false,
            minimum: None,
            maximum: None,
            default: None,
            replace_all: None,
            format: None,
            required: false,
            encrypt: false,
            replicate: false,
            r#match: None,
            on_unknown: None,
            r#enum: None,
            unique: false,
            links: None,
        }
    }
}

pub struct Models {
    pub name: String,
    pub fields: ModelFields,
}

impl Default for Models {
    fn default() -> Self {
        Models {
            name: String::new(),
            fields: ModelFields::default(),
        }
    }
}

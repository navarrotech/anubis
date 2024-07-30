// Copyright © 2024 Navarrotech

#[derive(PartialEq)]
pub enum ModelKind {
    String,
    Number,
    Float,
    Boolean,
    DateTime,
}

#[derive(PartialEq)]
pub enum UseOption {
    Uuid,
    Unique,
    OwnerLink,
    CreatedAt,
    UpdatedAt,
}

#[derive(PartialEq)]
pub enum FormatChoice {
    Email,
    Phone,
    Password,
    Secret,
}

#[derive(PartialEq)]
pub enum RelationshipMode {
    OneToOne,
    OneToMany,
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
    pub use_method: Option<UseOption>,
    pub format: Option<FormatChoice>,

    // Number fields
    pub minimum: Option<u32>,
    pub maximum: Option<u32>,

    // Misc
    pub replace_all: Option<String>,
    pub on_unknown: Option<String>,
    pub use_match: Option<String>,
    pub use_enum: Option<Vec<String>>,
    pub links: Option<String>,
}

impl Default for ModelFields {
    fn default() -> Self {
        ModelFields {
            name: String::new(),
            kind: ModelKind::String,
            use_method: None,
            primary_key: false,
            minimum: None,
            maximum: None,
            default: None,
            replace_all: None,
            format: None,
            required: false,
            encrypt: false,
            replicate: false,
            use_match: None,
            on_unknown: None,
            use_enum: None,
            unique: false,
            links: None,
        }
    }
}

pub struct Models {
    pub name: String,
    pub mode: RelationshipMode,
    pub fields: Vec<ModelFields>,
}

impl Default for Models {
    fn default() -> Self {
        Models {
            name: String::new(),
            mode: RelationshipMode::OneToOne,
            fields: Vec::from([ModelFields::default()]),
        }
    }
}

// Copyright © 2024 Navarrotech

enum ModelType {
  String,
  Number,
  Boolean,
}

enum UseOption {
  Uuid,
  CreatedAt,
  UpdatedAt,
}

enum FormatChoice {
  Email,
  Phone,
  Password,
}

pub struct ModelFields {
  name: String,
  type: ModelType,
  use: Option<UseOption>,
  primary_key: Option<bool>,
  min: Option<i32>,
  max: Option<i32>,
  minimum: Option<i32>,
  maximum: Option<i32>,
  default: Option<String>,
  replaceAll: Option<bool>,
  format: Option<FormatChoice>,
  required: Option<bool>,
  encrypt: Option<bool>,
  replicate: Option<bool>,
  match: Option<String>,
  on_unknown: Option<String>,
  enum: Option<Vec<String>>,
  unique: Option<bool>,
  links: Option<String>,
}

pub struct models {
  fields: ModelFields
}

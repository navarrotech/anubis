// Copyright © 2024 Navarrotech

use clap::Args;

use crate::cli::parse::parse_schema_yaml;
use crate::schema::AnubisSchema;

#[derive(Args)]
pub struct ValidateArgs {}

pub fn validate() {
    let schema = parse_schema_yaml();
    println!("Validating...");
}

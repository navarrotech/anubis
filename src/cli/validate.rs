// Copyright © 2024 Navarrotech

use clap::Args;

use crate::cli::parse::parse_schema_yaml;
use crate::schema::AnubisSchema;

#[derive(Args)]
pub struct ValidateArgs {
    #[clap(short = 'd', long, default_value = "")]
    pub directory: String
}

pub fn validate(args: &ValidateArgs) -> AnubisSchema {
    let root_directory = std::env::current_dir().unwrap().join(&args.directory);
    let schema = parse_schema_yaml(root_directory);
    println!("Validating...");
    schema
}

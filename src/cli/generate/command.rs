// Copyright © 2024 Navarrotech

// Lib
use clap::Args;

// Core structs
use crate::schema::AnubisSchema;

// Setup sub-functions
use crate::cli::generate::protobufs::generate_protobufs;

#[derive(Args)]
pub struct GenerateArgs {
    #[clap(short = 'd', long, default_value = "")]
    pub directory: String,
}

pub fn generate(schema: &AnubisSchema, args: &GenerateArgs) {
    println!("Generating project...");
    generate_protobufs(schema);
    println!("Project generated successfully!");
}

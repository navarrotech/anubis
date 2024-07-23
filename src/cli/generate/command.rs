// Copyright © 2024 Navarrotech

// Lib
use clap::Args;

// Core structs
use crate::schema::AnubisSchema;

// Setup sub-functions
use crate::cli::generate::protobufs::{
    generate_common_protobuf, generate_protobufs, generate_root_protobuf,
};

#[derive(Args)]
pub struct GenerateArgs {}

pub fn generate(schema: &AnubisSchema, args: &GenerateArgs) {
    println!("Generating project...");
    generate_common_protobuf(schema);
    generate_root_protobuf(schema);
    generate_protobufs(schema);
    println!("Project generated successfully!");
}

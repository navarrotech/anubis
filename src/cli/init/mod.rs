// Copyright © 2024 Navarrotech

// Lib
use clap::Args;
use std::io;

// Setup sub-functions
use crate::cli::init::setup::setup_directories;
use crate::cli::relics::anubis_schema::setup_anubis_schema;
use crate::cli::relics::cicd::setup_cicd;
use crate::schema::AnubisSchema;

#[derive(Args)]
pub struct InitArgs {
    #[clap(short = 'd', long, default_value = "")]
    pub directory: String,
    #[clap(default_value = "My Project")]
    pub name: String,

    #[clap(short = 'c', long = "copyright", default_value = "")]
    pub copy: String,
}

pub fn init(schema: &AnubisSchema) -> io::Result<()> {
    println!("Running initialization...");

    // Setup base directories
    setup_directories(schema.install_directory.as_path())?;

    // Setup base Anubis.yaml Schema
    setup_anubis_schema(schema);

    // Have the user choose which CI/CD they want to use
    setup_cicd(schema);

    // TODO: Create Dockerfiles
    // TODO: Create README

    Ok(())
}

pub mod setup;

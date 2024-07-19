// Copyright © 2024 Navarrotech

// Lib
use clap::Args;
use std::io;

// Setup sub-functions
use crate::cli::init::setup::setup_directories;
use crate::cli::relics::cicd::setup_cicd;
use crate::schema::AnubisSchema;

#[derive(Args)]
pub struct InitArgs {
    #[clap(short = 'd', long, default_value = "")]
    pub directory: String,
    #[clap(default_value = "My Project")]
    pub name: String,
}

pub fn init(schema: &AnubisSchema) -> io::Result<()> {
    println!("Running initialization...");

    // Have the user choose which CI/CD they want to use
    setup_cicd(schema);

    // Setup base directories
    setup_directories(schema.install_directory.as_path())?;

    // TODO: Create schema file
    // TODO: Create Dockerfiles
    // TODO: Create README

    Ok(())
}

pub mod setup;

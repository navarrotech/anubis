// Copyright © 2024 Navarrotech

use std::io;

use crate::cli::init::setup::setup_directories;
use clap::Args;

#[derive(Args)]
pub struct InitArgs {
    #[clap(short = 'd', long, default_value = "")]
    directory: String,
    #[clap(default_value = "My Project")]
    name: String,
}

pub fn init(args: InitArgs) -> io::Result<()> {
    println!("Running initialization...");

    // Have the user choose which CI/CD they want to use

    // Setup base directories
    setup_directories(&args.directory)?;

    // TODO: Create schema file
    // TODO: Create Dockerfiles
    // TODO: Create CI/CD
    // TODO: Create README

    Ok(())
}

pub mod init;

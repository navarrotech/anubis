// Copyright © 2024 Navarrotech

use std::io;

use crate::cli::init::setup::setup;
use clap::Args;

#[derive(Args)]
pub struct InitArgs {
    #[clap(short = 'd', long, default_value = "")]
    directory: String,
}

pub fn init(args: InitArgs) -> io::Result<()> {
    println!("Running initialization...");

    // Setup base directories
    setup(&args.directory)?;

    Ok(())
}

pub mod init;

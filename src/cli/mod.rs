// Copyright © 2024 Navarrotech

use std::io;

use clap::Args;
use crate::cli::init::setup::setup;

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

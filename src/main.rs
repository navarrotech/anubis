// Copyright © 2024 Navarrotech

use crate::cli::init::{init, InitArgs};
use crate::schema::AnubisSchema;
use std::env;
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Alex Navarro")]
#[command(name = "cargo", bin_name = "cargo")]
enum CargoCli {
    Init(InitArgs),
}

fn main() -> std::io::Result<()> {
    let cli = CargoCli::parse();

    match cli {
        CargoCli::Init(args) => {
            let schema = AnubisSchema {
                project_name: args.name.clone(),
                install_directory: env::current_dir()?.join(&args.directory),
                copyright_header: String::from(""),
            };
            init(&schema)?;
        }
    }

    Ok(())
}
mod cli;
mod schema;

// Copyright © 2024 Navarrotech

use crate::cli::{init, InitArgs};
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
            init(args)?;
        }
    }

    Ok(())
}

mod cli;

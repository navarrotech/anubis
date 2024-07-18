// Copyright © 2024 Navarrotech

use clap::{Parser, Args};

#[derive(Parser)]
#[command(name = "cargo", bin_name = "cargo")]
enum CargoCli {
    ExampleDerive(ExampleDeriveArgs),
    Init(InitArgs),
}

#[derive(Args)]
#[command(version, about, long_about = None)]
struct ExampleDeriveArgs {
    #[arg(long)]
    manifest_path: Option<std::path::PathBuf>,
}

#[derive(Args)]
struct InitArgs {
    // Add any arguments you need for the init command here
}

fn init(_args: InitArgs) {
    println!("Running init command");
    // Add your initialization logic here
}

fn main() {
    let cli = CargoCli::parse();

    match cli {
        CargoCli::ExampleDerive(args) => {
            println!("{:?}", args.manifest_path);
        }
        CargoCli::Init(args) => {
            init(args);
        }
    }
}

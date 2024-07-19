// Copyright © 2024 Navarrotech

// Lib
use chrono::Datelike;
use clap::Parser;
use cli::validate::{validate, ValidateArgs};
use std::env;

// Dialoguer
use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;

// Custom modules
use crate::cli::init::{init, InitArgs};
use crate::schema::AnubisSchema;

#[derive(Parser)]
#[clap(version = "1.0", author = "Alex Navarro")]
#[command(name = "cargo", bin_name = "cargo")]
enum CargoCli {
    Init(InitArgs),
    Validate(ValidateArgs),
}

fn main() -> std::io::Result<()> {
    let cli = CargoCli::parse();

    let now = chrono::Utc::now();
    let year = now.year();

    match cli {
        CargoCli::Init(args) => {
            let project_name: String = match args.name.is_empty() {
                true => Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Project name: ")
                    .default(String::from(""))
                    .interact_text()
                    .unwrap(),
                false => args.name,
            };

            let copyright_header: String = match args.copy.is_empty() {
                true => Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Copyright header: (Leave blank for none)")
                    .with_initial_text(String::from("Copyright © {YYYY} MyCompany"))
                    .interact_text()
                    .unwrap(),
                false => args.copy,
            };

            let copyright_header_formatted = copyright_header.replace("{YYYY}", &year.to_string());

            let schema = AnubisSchema {
                project_name,
                copyright_header,
                copyright_header_formatted,
                install_directory: env::current_dir()?.join(&args.directory),
                ..Default::default()
            };

            init(&schema)?;
        }
        CargoCli::Validate(_) => {
            validate();
            println!("Your Anubis.yaml file is valid!");
        }
    }

    Ok(())
}
mod cli;
mod schema;

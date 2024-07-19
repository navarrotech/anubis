// Copyright © 2024 Navarrotech

// Lib
use clap::Parser;
use std::env;
use chrono::Datelike;

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
                false => args.name
            };

            let copyright_header: String = match args.copy.is_empty() {
                true => Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Copyright header: (Leave blank for none)")
                    .default(String::from("Copyright © {YYYY} Company Name"))
                    .interact_text()
                    .unwrap(),
                false => args.copy
            };

            let copyright_header_formatted = copyright_header.replace("{YYYY}", &year.to_string());

            let mut schema = AnubisSchema::default();

            schema.project_name = project_name;
            schema.copyright_header = copyright_header;
            schema.copyright_header_formatted = copyright_header_formatted;
            schema.install_directory = env::current_dir()?.join(&args.directory);

            init(&schema)?;
        }
    }

    Ok(())
}
mod cli;
mod schema;

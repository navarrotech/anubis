// Copyright © 2024 Navarrotech

// Lib
use clap::Args;
use std::process::Command;
use std::thread;

// Core structs
use crate::schema::AnubisSchema;

#[derive(Args)]
pub struct InstallDependenciesArgs {
    #[clap(short = 'd', long, default_value = "")]
    pub directory: String
}

pub fn install_dependencies(schema: &AnubisSchema, args: &InstallDependenciesArgs) {

  // Run command to cd frontend && yarn install
  let frontend_directory = schema.install_directory.clone().join("frontend");

  let install_frontend = thread::spawn(move || {
    let status = Command::new("yarn")
        .arg("install")
        .current_dir(frontend_directory)
        .status()
        .expect("Failed to run yarn install in project1");

    if status.success() {
        println!("yarn install completed successfully in project1");
    } else {
        println!("yarn install failed in project1");
    }
  });

  
  // Run command to cd backend && cargo install
  let backend_directory = schema.install_directory.clone().join("api");
  let install_backend = thread::spawn(move || {
    let status = Command::new("cargo")
        .arg("build")
        .current_dir(backend_directory)
        .status()
        .expect("Failed to run cargo build in project1");

    if status.success() {
        println!("cargo build completed successfully in project1");
    } else {
        println!("cargo build failed in project1");
    }
  });
}

// Copyright © 2024 Navarrotech

use std::path::PathBuf;

pub struct AnubisSchema {
    pub project_name: String,
    pub description: String,
    pub version: String,

    pub install_directory: PathBuf,
    pub copyright_header: String,
    pub copyright_header_formatted: String,
}

impl Default for AnubisSchema {
    fn default() -> Self {
        let default_install_directory = std::env::current_dir().unwrap();
        AnubisSchema {
            project_name: String::from(""),
            description: String::from(""),
            version: String::from("1.0.0"),
            install_directory: default_install_directory,
            copyright_header: String::from(""),
            copyright_header_formatted: String::from(""),
        }
    }
}

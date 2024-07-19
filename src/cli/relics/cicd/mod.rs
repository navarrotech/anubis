// Copyright © 2024 Navarrotech

// Core
use std::fmt;

// Lib
use dialoguer::{theme::ColorfulTheme, Select};

// Crates
use crate::cli::relics::cicd::circleci::setup_circleci;
use crate::cli::relics::cicd::github_actions::setup_github_actions;
use crate::schema::AnubisSchema;

#[derive(Debug, Clone)]
enum CICDProvider {
    GitHubActions,
    GitLabCI,
    CircleCI,
    Skip,
}

// Implement a method to return all variants
impl CICDProvider {
    fn all() -> Vec<CICDProvider> {
        vec![
            CICDProvider::GitHubActions,
            CICDProvider::GitLabCI,
            CICDProvider::CircleCI,
            CICDProvider::Skip,
        ]
    }
}

// Implement Display trait to convert enum to string
impl fmt::Display for CICDProvider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CICDProvider::GitHubActions => write!(f, "GitHub Actions"),
            CICDProvider::GitLabCI => write!(f, "GitLab CI"),
            CICDProvider::CircleCI => write!(f, "Circle CI"),
            CICDProvider::Skip => write!(f, "Skip"),
        }
    }
}

pub fn setup_cicd(schema: &AnubisSchema) {
    println!("Setting up CI/CD pipeline...");

    let items: Vec<String> = CICDProvider::all()
        .iter()
        .map(|provider| provider.to_string())
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which CI/CD do you wish to use?")
        .items(&items)
        .default(0)
        .interact()
        .unwrap();

    // Handle the selection
    let option = CICDProvider::all()[selection].clone();

    match option {
        CICDProvider::GitHubActions => {
            setup_github_actions(schema);
        }
        CICDProvider::GitLabCI => {
            // TODO: Implement GitLab CI setup
            println!("Setting up GitLab CI...");
        }
        CICDProvider::CircleCI => {
            setup_circleci(schema);
        }
        CICDProvider::Skip => {
            println!("Skipping CI/CD setup...");
        }
    }

    println!("Selected: {:?}", option);
    println!("Done!");
}

pub mod circleci;
pub mod github_actions;

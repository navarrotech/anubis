// Copyright © 2024 Navarrotech

use std::env;
use std::fs;
use std::io;

const DIRS: &[&str] = &["frontend/src", "backend/src", ".github/workflows"];

pub fn setup_directories(base_path: &String) -> io::Result<()> {
    let current_dir = env::current_dir()?;
    let current_dir = current_dir.join(base_path);

    for &dir in DIRS {
        let path = current_dir.join(dir);
        if !path.exists() {
            fs::create_dir_all(&path)?;
            println!("Created directory: {}", path.display());
        }
    }

    Ok(())
}

#[cfg(test)]
mod setup_tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_setup_creates_directories_in_current_dir() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path().to_str().unwrap().to_string();

        setup_directories(&base_path).unwrap();

        for &dir in DIRS {
            let path = temp_dir.path().join(dir);
            assert!(path.exists(), "Directory should exist: {}", path.display());
        }
    }

    #[test]
    fn test_setup_creates_directories_in_specified_dir() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir
            .path()
            .join("test_subdir")
            .to_str()
            .unwrap()
            .to_string();

        setup_directories(&base_path).unwrap();

        for &dir in DIRS {
            let path = temp_dir.path().join("test_subdir").join(dir);
            assert!(path.exists(), "Directory should exist: {}", path.display());
        }
    }
}

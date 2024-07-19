// Copyright © 2024 Navarrotech

use std::fs;
use std::io;
use std::path::Path;

const DIRS: &[&str] = &[
    "frontend/src",
    "backend/src",
    ".github/workflows",
    ".anubis",
];

pub fn setup_directories(base_path: &Path) -> io::Result<()> {
    for &dir in DIRS {
        let path = base_path.join(dir);
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
        let base_path = temp_dir.path();

        setup_directories(base_path).unwrap();

        for &dir in DIRS {
            let path = temp_dir.path().join(dir);
            assert!(path.exists(), "Directory should exist: {}", path.display());
        }
    }

    #[test]
    fn test_setup_creates_directories_in_specified_dir() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path().join("test_subdir");

        setup_directories(base_path.as_path()).unwrap();

        for &dir in DIRS {
            let path = temp_dir.path().join("test_subdir").join(dir);
            assert!(path.exists(), "Directory should exist: {}", path.display());
        }
    }
}

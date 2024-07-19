// Copyright © 2024 Navarrotech

use crate::schema::AnubisSchema;

pub fn setup_github_actions(schema: &AnubisSchema) {
    println!("Setting up GitHub Actions...");
    let content = create_github_actions(schema);

    // Create the file path
    let file = schema
        .install_directory
        .clone()
        .join(".github/workflows/build.yml");

    // Ensure all parent directories exist
    std::fs::create_dir_all(file.parent().unwrap()).expect("Unable to create parent directories");

    // Write the file
    std::fs::write(file, content).expect("Unable to write github actions file");
}

pub fn create_github_actions(schema: &AnubisSchema) -> String {
    let copyright = if schema.copyright_header_formatted.is_empty() {
        schema.copyright_header_formatted.clone()
    } else {
        format!("# {}", schema.copyright_header_formatted)
    };

    format!("{copyright_header}name: Build & Test {project_name}


#  This is a generated relic by Anubis. It sets up a GitHub Actions workflow that will:
#  - Cache the Cargo registry and index
#  - Ensure unit tests are ran
#  - Build the project in release mode
#  - Push the build artifacts to GitHub
#  - Optionally create a Docker image and push to Docker Hub
#  
#  This file is an Anubis relic, meaning it was only auto-generated from running 'anubis init' and selecting GitHub Actions.
#  You may safely modify this file as much as you want, and Anubis will not touch this file again.
#  
#  To regenerate this file and restore all defaults, you can run:
#  `anubis relics create github-actions`


on:
  push:
    branches:
      - main
  pull_request:
    branches:
      - main
  workflow_dispatch:

jobs:
  build-rust:
    runs-on: ubuntu-latest

    container:
      image: rust:latest

    needs: [ build-frontend ]

    steps:

      # Setup:
      - name: Checkout code
        uses: actions/checkout@v4

      # Cache dependencies
      - name: Cache Cargo registry
        uses: actions/cache@v4
        id: cache-cargo-registry
        with:
          path: ~/.cargo/registry
          key: ${{{{ runner.os }}}}-cargo-registry-${{{{ hashFiles('**/Cargo.lock') }}}}
          restore-keys: |
            ${{{{ runner.os }}}}-cargo-registry-

      # Cache index
      - name: Cache Cargo index
        uses: actions/cache@v4
        id: cache-cargo-index
        with:
          path: ~/.cargo/git
          key: ${{{{ runner.os }}}}-cargo-git-${{{{ hashFiles('**/Cargo.lock') }}}}
          restore-keys: |
            ${{{{ runner.os }}}}-cargo-git-

      # Install dependencies
      - name: Install dependencies
        if: steps.cache-cargo-registry.outputs.cache-hit != 'true' && steps.cache-cargo-index.outputs.cache-hit != 'true'
        run: |
          cd backend
          cargo fetch

      # Optional: Require Rust formatting
      - name: Check Rust formatting
        run: |
          cd backend
          cargo fmt --all -- --check

      # Run unit tests
      - name: Run Rust unit tests
        run: |
          cd backend
          cargo test

      # Build the release candidate
      - name: Build for release
        run: |
          cd backend
          cargo build --release

      # Download the frontend assets to be included in the Docker builds
      - uses: actions/download-artifact@v4
        with:
          name: frontend-assets
          path: static/
  
      # Upload the build artifacts
      - name: Archive build artifacts
        uses: actions/upload-artifact@v4

        # Only upload artifacts on main branch
        # Ideal so your pull requests don't get cluttered with artifacts
        if: github.ref == 'refs/heads/main'

        with:
          path: target/release
          if-no-files-found: 'warn'
          compression-level: 6

      # Save cargo registry cache
      - name: Cache cargo registry
        uses: actions/cache/save@v4
        with:
          path: ~/.cargo/registry
          key: ${{{{ runner.os }}}}-cargo-registry-${{{{ hashFiles('**/Cargo.lock') }}}}

      # Save cargo index cache
      - name: Cache cargo index
        uses: actions/cache/save@v4
        with:
          path: ~/.cargo/git
          key: ${{{{ runner.os }}}}-cargo-git-${{{{ hashFiles('**/Cargo.lock') }}}}

      # Optional, build a Docker image for the backend service and push to Docker Hub
      # Only enabled if you have Docker Hub credentials set in your Github Secrets
      - name: Log in to Docker Hub
        uses: docker/login-action@v3
        if: github.ref == 'refs/heads/main' && github secrets.DOCKER_HUB_USERNAME != '' && secrets.DOCKER_HUB_ACCESS_TOKEN != ''
        with:
          username: ${{{{ secrets.DOCKER_HUB_USERNAME }}}}
          password: ${{{{ secrets.DOCKER_HUB_ACCESS_TOKEN }}}}

      # Build and push Docker image
      - name: Build and push Docker image
        if: github.ref == 'refs/heads/main' && secrets.DOCKER_HUB_USERNAME != '' && secrets.DOCKER_HUB_ACCESS_TOKEN != ''
        run: |
          cd backend

          # Get the short Git hash
          GIT_HASH=$(git rev-parse --short HEAD)
          
          # Build the Docker image and tag it as latest
          docker build . -t ${{{{ secrets.DOCKER_HUB_USERNAME }}}}/{project_name}-backend-rust:latest -f ./Dockerfile
          
          # Push the latest tag
          docker push ${{{{ secrets.DOCKER_HUB_USERNAME }}}}/{project_name}-backend-rust:latest
          
          # Tag the image with the short Git hash
          docker tag ${{{{ secrets.DOCKER_HUB_USERNAME }}}}/{project_name}-backend-rust:latest ${{{{ secrets.DOCKER_HUB_USERNAME }}}}/{project_name}-backend-rust:$GIT_HASH
          
          # Push the image with the Git hash tag
          docker push ${{{{ secrets.DOCKER_HUB_USERNAME }}}}/{project_name}-backend-rust:$GIT_HASH

      # Optional, remove the frontend assets artifact
      # This can be useful if you don't want to keep the frontend assets after building the Docker image
      # This can help save a lot of space in your Github Actions storage
      - name: Remove frontend assets artifact
        uses: geekyeggo/delete-artifact@v5
        with:
            name: my-artifact

  build-frontend:
    runs-on: ubuntu-latest

    container:
      image: node:latest

    steps:

      # Setup:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Restore node_modules cache
        id: cache-frontend-deps
        uses: actions/cache@v4
        with:
          path: frontend/node_modules
          key: ${{{{ runner.os }}}}-frontend-dependencies-${{{{ hashFiles('**/yarn.lock') }}}}

      # Install dependencies & make assets:
      - name: Install dependencies
        if: steps.cache-frontend-deps.outputs.cache-hit != 'true'
        run: |
          cd frontend
          yarn install

      # Testing:
      - name: Run unit tests
        run: |
          cd frontend
          yarn test

      # Optional: Require ESLint checks to pass before building
      - name: Ensure eslint
        run: |
          cd frontend
          yarn lint

      # Build:
      - name: Build frontend assets
        if: github.ref == 'refs/heads/main'
        run: |
          cd frontend
          yarn build

      - name: Save for next step
        uses: actions/upload-artifact@v4
        if: github.ref == 'refs/heads/main'
        with:
          name: frontend-assets
          path: frontend/dist

      # Save cache
      - name: Save node_modules cache
        uses: actions/cache/save@v4
        with:
          path: frontend/node_modules
          key: ${{{{ runner.os }}}}-frontend-dependencies-${{{{ hashFiles('**/yarn.lock') }}}}

    ", project_name = schema.project_name, copyright_header = copyright)
}

#[cfg(test)]
mod check_github_actions {
    use super::*;
    use crate::schema::AnubisSchema;
    use serde_yaml;
    use serde_yaml::Error;
    use tempfile::tempdir;

    fn is_valid_yaml(yaml_str: &str) -> Result<(), Error> {
        serde_yaml::from_str::<serde_yaml::Value>(yaml_str).map(|_| ())
    }

    #[test]
    fn ensure_github_actions_yaml_is_valid() {
        let mut test_schema = AnubisSchema::default();
        test_schema.project_name = "test".to_string();

        let content = create_github_actions(&test_schema);

        assert!(is_valid_yaml(content.as_str()).is_ok());
    }

    #[test]
    fn ensure_github_actions_yaml_is_valid_with_project_name_spaces() {
        let mut test_schema = AnubisSchema::default();
        test_schema.project_name = "name with spaces".to_string();

        let content = create_github_actions(&test_schema);

        assert!(is_valid_yaml(content.as_str()).is_ok());
    }

    #[test]
    fn ensure_github_actions_yaml_is_valid_with_copyright_header() {
        let mut test_schema = AnubisSchema::default();
        test_schema.copyright_header = String::from("// Copyright © {YYYY} Navarrotech");
        test_schema.copyright_header_formatted = String::from("// Copyright © 2024 Navarrotech");

        let content = create_github_actions(&test_schema);

        assert!(is_valid_yaml(content.as_str()).is_ok());
    }

    #[test]
    fn ensure_github_actions_writes_the_file() {
        let temp_directory = tempdir().unwrap().into_path();

        let mut test_schema = AnubisSchema::default();
        test_schema.project_name = "Anubis Test".to_string();
        test_schema.copyright_header = String::from("// Copyright © {YYYY} Navarrotech");
        test_schema.copyright_header_formatted = String::from("// Copyright © 2024 Navarrotech");
        test_schema.install_directory = temp_directory.clone();

        setup_github_actions(&test_schema);

        let file_path = temp_directory.join(".github/workflows/build.yml");
        assert!(file_path.exists());

        let content = std::fs::read_to_string(file_path).unwrap();
        assert!(is_valid_yaml(content.as_str()).is_ok());
    }
}

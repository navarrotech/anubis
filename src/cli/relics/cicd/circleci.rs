// Copyright © 2024 Navarrotech

use crate::schema::AnubisSchema;

pub fn setup_circleci(schema: &AnubisSchema) -> String {
    println!("Setting up CircleCI...");
    format!("{copyright_header}version: 2.1

#  This is a generated relic by Anubis. It sets up a CircleCI Actions workflow that will:
#  - Cache the Cargo registry and index
#  - Ensure unit tests are ran
#  - Build the project in release mode
#  - Push the build artifacts to CircleCI
#  - Optionally create a Docker image and push to Docker Hub
#  
#  This file is an Anubis relic, meaning it was only auto-generated from running 'anubis init' and selecting CircleCI Actions.
#  You may safely modify this file as much as you want, and Anubis will not touch this file again.
#  
#  To regenerate this file and restore all defaults, you can run:
#  `anubis relics create circleci-actions`

executors:
  rust-executor:
    working_directory: ~/app
    docker:
      - image: cimg/rust:latest

  node-executor:
    working_directory: ~/app
    docker:
      - image: cimg/node:latest

jobs:
  build-rust:
    executor: rust-executor
    steps:
      - checkout

      # Cache dependencies
      - restore_cache:
          keys:
            - cargo-registry-{{ checksum \"Cargo.lock\" }}

      - restore_cache:
          keys:
            - cargo-index-{{ checksum \"Cargo.lock\" }}

      # Install dependencies
      - run:
          name: Install dependencies
          command: |
            cd backend
            cargo fetch

      # Optional: Require Rust formatting
      - run:
          name: Check Rust formatting
          command: |
            cd backend
            cargo fmt --all -- --check

      # Run unit tests
      - run:
          name: Run Rust unit tests
          command: |
            cd backend
            cargo test

      # Build the release candidate
      - run:
          name: Build for release
          command: |
            cd backend
            cargo build --release

      # Save build artifacts
      - persist_to_workspace:
          root: .
          paths:
            - backend/target/release
          when:
            equal: [ main, << pipeline.git.branch >> ]

      # Save cache
      - save_cache:
          key: cargo-registry-{{ checksum \"Cargo.lock\" }}
          paths:
            - ~/.cargo/registry

      - save_cache:
          key: cargo-index-{{ checksum \"Cargo.lock\" }}
          paths:
            - ~/.cargo/git

      # Optional, build a Docker image for the backend service and push to Docker Hub
      - setup_remote_docker:
          docker_layer_caching: true
          when:
            equal: [ main, << pipeline.git.branch >> ]

      - run:
          name: Build and push Docker image
          when:
            equal: [ main, << pipeline.git.branch >> ]
          command: |
            cd backend

            # Get the short Git hash
            GIT_HASH=$(git rev-parse --short HEAD)

            # Log in to Docker Hub
            docker login -u $DOCKER_USERNAME -p $DOCKER_PASSWORD

            # Build the Docker image and tag it as latest
            docker build . -t $DOCKER_HUB_USERNAME/{project_name}-backend-rust:latest  -f ./Dockerfile

            # Push the latest tag
            docker push $DOCKER_HUB_USERNAME/{project_name}-backend-rust:latest

            # Tag the image with the short Git hash
            docker tag $DOCKER_HUB_USERNAME/{project_name}-backend-rust:latest $DOCKER_HUB_USERNAME/{project_name}-backend-rust:$GIT_HASH

            # Push the image with the Git hash tag
            docker push $DOCKER_HUB_USERNAME/{project_name}-backend-rust:$GIT_HASH

  build-frontend:
    executor: node-executor
    steps:
      - checkout
  
      # Cache dependencies
      - restore_cache:
          keys:
            - node-dependencies-{{ checksum \"yarn.lock\" }}
  
      # Install dependencies & make assets
      - run:
          name: Install dependencies
          command: |
            cd frontend
            yarn install
  
      # Run unit tests
      - run:
          name: Run unit tests
          command: |
            cd frontend
            yarn test
  
      # Optional: Require ESLint checks to pass before building
      - run:
          name: Ensure eslint
          command: |
            cd frontend
            yarn lint
  
      # Build frontend assets
      - run:
          name: Build frontend assets
          command: |
            cd frontend
            yarn build
  
      # Save build artifacts
      - persist_to_workspace:
          root: .
          paths:
            - frontend/dist
          when:
            equal: [ main, << pipeline.git.branch >> ]
  
      # Save cache
      - save_cache:
          key: node-dependencies-{{ checksum \"yarn.lock\" }}
          paths:
            - frontend/node_modules

workflows:
  build_and_test:
    jobs:
      - build-frontend
      - build-rust:
          requires:
            - build-frontend

  ", project_name = schema.project_name, copyright_header = schema.copyright_header)
}

#[cfg(test)]
mod check_circleci {
    use super::*;
    use crate::schema::AnubisSchema;
    use serde_yaml;
    use serde_yaml::Error;
    use std::path::PathBuf;

    fn is_valid_yaml(yaml_str: &str) -> Result<(), Error> {
        serde_yaml::from_str::<serde_yaml::Value>(yaml_str).map(|_| ())
    }

    #[test]
    fn ensure_circleci_yaml_is_valid() {
        let test_schema = AnubisSchema {
            project_name: "test".to_string(),
            copyright_header: String::from(""),
            install_directory: PathBuf::from("."),
        };
        let content = setup_circleci(&test_schema);

        assert!(is_valid_yaml(content.as_str()).is_ok());
    }

    #[test]
    fn ensure_circleci_yaml_is_valid_with_project_name_spaces() {
        let test_schema = AnubisSchema {
            project_name: "name with spaces".to_string(),
            copyright_header: String::from(""),
            install_directory: PathBuf::from("."),
        };
        let content = setup_circleci(&test_schema);

        assert!(is_valid_yaml(content.as_str()).is_ok());
    }

    #[test]
    fn ensure_circleci_yaml_is_valid_with_copyright_header() {
        let test_schema = AnubisSchema {
            project_name: "test".to_string(),
            copyright_header: String::from("// Copyright © 2024 Navarrotech"),
            install_directory: PathBuf::from("."),
        };
        let content = setup_circleci(&test_schema);

        assert!(is_valid_yaml(content.as_str()).is_ok());
    }
}

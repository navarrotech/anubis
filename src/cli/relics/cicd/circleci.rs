// Copyright © 2024 Navarrotech

use crate::schema::AnubisSchema;

pub fn create_circleci(schema: &AnubisSchema) -> String {
    let project_name = schema.project_name.replace(' ', "-");

    format!("version: 2.1

# This CircleCI flow will:
#  - Cache the Cargo registry and index
#  - Ensure unit tests are ran for frontend and backend
#  - Ensure lint checks are ran for frontend and backend
#  - Build the project in release mode
#  - Push the build artifacts to CircleCI storage
#  - Optionally create a Docker image and push to Docker Hub if Docker Hub credentials are set
#  
#  To regenerate this file and restore all defaults, you can run:
#  `anubis relics create circleci-actions`

executors:
  rust:
    working_directory: ~/app
    docker:
      - image: rust:latest

  node:
    working_directory: ~/app
    docker:
      - image: node:latest

  ubuntu:
    working_directory: ~/app
    docker:
      - image: ubuntu:latest

# Define reusable commands for the frontend and backend
commands:
  setup_frontend:
    steps:
      # Cache dependencies
      - restore_cache:
          keys:
            - node-dependencies-{{{{ checksum \"yarn.lock\" }}}}
  
      # Install dependencies & make assets
      - run:
          name: Install dependencies
          command: |
            cd frontend
            yarn install
            
      # Save cache
      - save_cache:
          key: node-dependencies-{{{{ checksum \"yarn.lock\" }}}}
          paths:
            - frontend/node_modules

  setup_api:
    steps:
      # Cache dependencies
      - restore_cache:
          keys:
            - cargo-registry-{{{{ checksum \"Cargo.lock\" }}}}

      - restore_cache:
          keys:
            - cargo-index-{{{{ checksum \"Cargo.lock\" }}}}

      # Install dependencies
      - run:
          name: Install dependencies
          command: |
            cd api
            cargo fetch

      # Save cache
      - save_cache:
          key: cargo-registry-{{{{ checksum \"Cargo.lock\" }}}}
          paths:
            - ~/.cargo/registry

      - save_cache:
          key: cargo-index-{{{{ checksum \"Cargo.lock\" }}}}
          paths:
            - ~/.cargo/git

jobs:
  test-frontend:
    executor: node

    steps:
      - checkout
      - setup_frontend
      
      # Run unit tests
      - run:
          name: Run unit tests
          command: |
            cd frontend
            yarn test

      # Check typescript
      - run:
          name: Check typescript
          command: |
            cd frontend
            yarn tsc
  
      # Optional: Require ESLint checks to pass before building
      - run:
          name: Ensure eslint
          command: |
            cd frontend
            yarn lint

  test-api:
    executor: rust

    steps:
      - checkout
      - setup_api

      # Run unit tests
      - run:
          name: Run Rust unit tests
          command: |
            cd api
            cargo test

      # Optional: Require Rust formatting
      - run:
          name: Check Rust formatting
          command: |
            cd api
            cargo fmt --all -- --check

      # Optional: Require Rust clippy checks
      - run:
          name: Check Rust formatting
          command: |
            cd api
            cargo add --dev clippy
            cargo clippy

  build-api:
    executor: rust
    steps:
      - checkout
      - setup_api

      # Build the release candidate
      - run:
          name: Build for release
          command: |
            cd api
            cargo build --release

      # Save build artifacts
      - store_artifacts:
          path: api/target/release
          destination: api-release

      - persist_to_workspace:
          root: .
          paths:
            - api/target/release

  build-frontend:
    executor: node
    steps:
      - checkout
      - setup_frontend

      # Build frontend assets
      - run:
          name: Build frontend assets
          command: |
            cd frontend
            yarn build

      # Upload artifacts
      - store_artifacts:
          path: frontend/dist
          destination: frontend

      # Save build artifacts
      - persist_to_workspace:
          root: .
          paths:
            - frontend/dist

  package-docker:
    executor: ubuntu
    steps:
      # Gather the build artifacts (frontend/dist and api/target/release)
      - attach_workspace:
          at: .

      # Build a Docker image for the api service and push to Docker Hub
      - setup_remote_docker:
          docker_layer_caching: true

      - run:
          name: Build and push Docker image
          when: always
          command: |
            if [ -n \"$DOCKER_USERNAME\" ] && [ -n \"$DOCKER_PASSWORD\" ]; then
              cd api

              # Get the short Git hash
              GIT_HASH=$(git rev-parse --short HEAD)

              # Log in to Docker Hub
              docker login -u $DOCKER_USERNAME -p $DOCKER_PASSWORD

              # Build the Docker image and tag it as latest
              docker build . -t $DOCKER_HUB_USERNAME/{project_name}api-rust:latest  -f ./Dockerfile

              # Push the latest tag
              docker push $DOCKER_HUB_USERNAME/{project_name}api-rust:latest

              # Tag the image with the short Git hash
              docker tag $DOCKER_HUB_USERNAME/{project_name}api-rust:latest $DOCKER_HUB_USERNAME/{project_name}api-rust:$GIT_HASH

              # Push the image with the Git hash tag
              docker push $DOCKER_HUB_USERNAME/{project_name}api-rust:$GIT_HASH
            else
              echo \"Environment variables [DOCKER_USERNAME, DOCKER_PASSWORD] for docker.io are not set. Skipping the step.\"
            fi

workflows:
  build_and_test:
    jobs:
      # Always test for linting, unit tests, and checking if it builds
      - test-frontend
      - test-api

      - build-frontend:
          requires:
            - test-frontend
          filters:
            branches:
              only: main

      - build-api:
          requires:
            - test-api
            - build-frontend
          filters:
            branches:
              only: main

      - package-docker:
          requires:
            - build-api
          filters:
            branches:
              only: main
  
", project_name = project_name)
}

#[cfg(test)]
mod check_circleci {
    use super::*;
    use crate::schema::AnubisSchema;
    use serde_yaml;
    use serde_yaml::Error;

    fn is_valid_yaml(yaml_str: &str) -> Result<(), Error> {
        serde_yaml::from_str::<serde_yaml::Value>(yaml_str).map(|_| ())
    }

    #[test]
    fn ensure_circleci_yaml_is_valid() {
        let mut test_schema = AnubisSchema::default();
        test_schema.project_name = "test".to_string();

        let content = create_circleci(&test_schema);

        assert!(is_valid_yaml(content.as_str()).is_ok());
    }

    #[test]
    fn ensure_circleci_yaml_is_valid_with_project_name_spaces() {
        let mut test_schema = AnubisSchema::default();
        test_schema.project_name = "name with spaces".to_string();

        let content = create_circleci(&test_schema);

        assert!(is_valid_yaml(content.as_str()).is_ok());
    }
}

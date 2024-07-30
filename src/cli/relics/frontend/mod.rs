// Copyright © 2024 Navarrotech

use eslint::create_eslint;
use frontend_core::generate_frontend_core;
use i18n::generate_i18next;
use i18n::generate_translation_json;
use package_json::create_package_json;
use scss::generate_scss;
use seo::generate_seo;
use tsconfig::create_tsconfig;
use tsconfig::create_tsconfig_node;
use vite::create_vite_config;
use vite::create_vitest_config;

use crate::cli::synthetics::index_html::create_frontend_html;
use crate::relics::write::write_relic;
use crate::schema::AnubisSchema;
use crate::synthetics::write::write_synthetic;

pub fn setup_frontend(schema: &AnubisSchema) {
    println!("Setting up frontend...");

    // Package.json
    write_relic(
        schema,
        &create_package_json(schema),
        &schema.install_directory.join("frontend/package.json"),
    );

    // Eslint.js
    write_relic(
        schema,
        &create_eslint(schema),
        &schema.install_directory.join("frontend/.eslintrc.cjs"),
    );

    // Tsconfig.json
    write_relic(
        schema,
        &create_tsconfig(),
        &schema.install_directory.join("frontend/tsconfig.json"),
    );

    // Tsconfig.node.json
    write_relic(
        schema,
        &create_tsconfig_node(),
        &schema.install_directory.join("frontend/tsconfig.node.json"),
    );

    // Vite.config.ts
    write_relic(
        schema,
        &create_vite_config(),
        &schema.install_directory.join("frontend/vite.config.ts"),
    );

    // Vite.config.ts
    write_relic(
        schema,
        &create_vitest_config(),
        &schema.install_directory.join("frontend/vitest.config.ts"),
    );

    // Index.html
    write_synthetic(
        schema,
        &create_frontend_html(schema),
        &schema.install_directory.join("frontend/index.html"),
    );

    generate_i18next(schema);
    generate_translation_json(schema);
    generate_frontend_core(schema);
    generate_scss(schema);
    generate_seo(schema);

}

pub mod eslint;
pub mod package_json;
pub mod tsconfig;
pub mod vite;
pub mod i18n;
pub mod frontend_core;
pub mod scss;
pub mod seo;

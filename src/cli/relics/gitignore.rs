// Copyright © 2024 Navarrotech

use crate::automatrons::write::write_automatron;
use crate::schema::AnubisSchema;

pub fn generate_gitignore(schema: &AnubisSchema) {
    let auth_protobuf = format!(
        r#"
# Unit testing results
test-results/

# Api
api/target/

# Frontend
node_modules/
yarn-error.log
"#
    );

    write_automatron(
        schema,
        &auth_protobuf,
        &schema.install_directory.join("./.gitignore"),
    );
}

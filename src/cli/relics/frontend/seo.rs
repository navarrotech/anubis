// Copyright © 2024 Navarrotech

use crate::relics::write::write_relic;
use crate::schema::AnubisSchema;

pub fn generate_seo(schema: &AnubisSchema) {
    generate_robots_txt(schema);
    generate_manifest(schema);
}

// TODO: Build this out
fn generate_robots_txt(schema: &AnubisSchema) {
    let auth_protobuf = format!(
        r#"
# https://www.robotstxt.org/robotstxt.html
User-agent: *
Disallow:

"#
    );

    write_relic(
        schema,
        &auth_protobuf,
        &schema.install_directory.join("frontend/public/robots.txt"),
    );
}

// TODO: Build this out
fn generate_manifest(schema: &AnubisSchema) {
    let auth_protobuf = format!(
        r#"
{{
  \"short_name\": \"{name}\",
  \"name\": \"{name}\",
  \"icons\": [
    {{
      \"src\": \"favicon.ico\",
      \"sizes\": \"64x64 32x32 24x24 16x16\",
      \"type\": \"image/x-icon\"
    }},
    {{
      \"src\": \"logo192.png\",
      \"type\": \"image/png\",
      \"sizes\": \"192x192\"
    }},
    {{
      \"src\": \"logo512.png\",
      \"type\": \"image/png\",
      \"sizes\": \"512x512\"
    }}
  ],
  \"start_url\": \".\",
  \"display\": \"standalone\",
  \"theme_color\": \"\#000000\",
  \"background_color\": \"\#ffffff\"
}}
"#,
        name = schema.project_name
    );

    write_relic(
        schema,
        &auth_protobuf,
        &schema
            .install_directory
            .join("frontend/public/manifest.json"),
    );
}

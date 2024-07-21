// Copyright © 2024 Navarrotech

use crate::schema::AnubisSchema;

pub fn get_copyright_header(schema: &AnubisSchema, file_name: &String) -> String {
    if schema.copyright_header_formatted.is_empty() {
        return String::from("");
    }

    let comment_type = get_comment_type(file_name);

    if !comment_type.is_empty() {
        return format!(
            "{comment} {copyright}\n\n",
            comment = comment_type,
            copyright = schema.copyright_header_formatted
        );
    }

    return String::from("");
}

pub fn get_file_type(file_name: &String) -> &str {
    // Determine the file type (JSON, YAML, JS, RS, etc.)
    let file_type: &str = match file_name.split('.').last() {
        Some(file_type) => file_type,
        None => "txt",
    };

    return file_type;
}

pub fn get_comment_type(file_name: &String) -> String {
    // Determine the file type (JSON, YAML, JS, RS, etc.)
    let file_type = get_file_type(file_name);

    match file_type {
        // Yaml
        "yml" | "yaml" => return String::from("#"),
        // Rust, Javascript & Typescript
        "rs" | "js" | "ts" | "tsx" => return String::from("//"),
        _ => (),
    }

    return String::from("");
}

// Copyright © 2024 Navarrotech

use std::path::PathBuf;

use crate::cli::common::get_comment_type;
use crate::cli::common::get_copyright_header;
use crate::cli::common::get_file_type;
use crate::schema::AnubisSchema;

pub fn write_automatron(schema: &AnubisSchema, content: &String, file_path: &PathBuf) {
    let mut super_content = content.clone();

    // Get file name
    let file_name = file_path
        .file_name()
        .expect("Invalid file path")
        .to_string_lossy()
        .to_string();

    let file_type = get_file_type(&file_name);

    // Write all parent directories in the path if they don't exist
    std::fs::create_dir_all(file_path.parent().unwrap()).expect(&format!(
        "Unable to create parent directories for {file_path}",
        file_path = file_path.display()
    ));

    let comment_type = get_comment_type(&file_name);
    let copyright = get_copyright_header(schema, &file_name);
    let preamble = format!(
"{comment} ////////////////////////////////////////////
{comment} // !! AUTO GENERATED FILE, DO NOT EDIT !! //
{comment} ////////////////////////////////////////////

{comment} This is a generated automatron file by Anubis. 
{comment} Automatrons are files that are 100% auto-generated regularly by Anubis.
{comment} It is not safe to edit this file directly, as your changes are extremly likely to be overwritten.",
        comment = comment_type
    );

    if file_type != "md" && file_type != "html" && file_type != "json" && file_type != "gitignore" {
        super_content = format!(
            "{copyright}\n{preamble}\n{content}",
            preamble = preamble,
            copyright = copyright,
            content = content
        );
    }

    // Write the content to the file
    std::fs::write(file_path, super_content).expect(&format!(
        "Unable to write {file_name} file",
        file_name = file_name
    ));
}

#[cfg(test)]
mod automatrons {
    use super::*;
    use tempfile::tempdir;

    fn generate_temp_file(content: &String, file_path: &PathBuf) -> PathBuf {
        let temp_directory = tempdir().unwrap().into_path();

        let mut test_schema = AnubisSchema::default();
        test_schema.project_name = "Anubis Test".to_string();
        test_schema.copyright_header = String::from("Copyright © {YYYY} Navarrotech");
        test_schema.copyright_header_formatted = String::from("Copyright © 2024 Navarrotech");
        test_schema.install_directory = temp_directory.clone();

        let path_upgraded = temp_directory.clone().join(file_path);

        write_automatron(&test_schema, &content, &path_upgraded);

        path_upgraded
    }

    #[test]
    fn ensure_automatron_headers_yaml() {
        let file_path = generate_temp_file(&String::from("Foo Bazz"), &PathBuf::from("test.yml"));

        assert!(file_path.exists());

        let file_contents = std::fs::read_to_string(file_path).unwrap();

        assert!(file_contents.starts_with("# Copyright © 2024 Navarrotech\n"));
        assert!(file_contents.contains("Foo Bazz"));
    }

    #[test]
    fn ensure_automatron_headers_javascript() {
        let file_path = generate_temp_file(
            &String::from("const foo = 'bar';"),
            &PathBuf::from("test.js"),
        );

        assert!(file_path.exists());

        let file_contents = std::fs::read_to_string(file_path).unwrap();

        assert!(file_contents.starts_with("// Copyright © 2024 Navarrotech\n"));
        assert!(file_contents.contains("const foo = 'bar';"));
    }

    #[test]
    fn ensure_automatron_headers_typescript() {
        let file_path = generate_temp_file(
            &String::from("const foo = 'bar';"),
            &PathBuf::from("test.ts"),
        );

        assert!(file_path.exists());

        let file_contents = std::fs::read_to_string(file_path).unwrap();

        assert!(file_contents.starts_with("// Copyright © 2024 Navarrotech\n"));
        assert!(file_contents.contains("const foo = 'bar';"));
    }

    #[test]
    fn ensure_automatron_headers_typescript_react() {
        let file_path = generate_temp_file(
            &String::from("const foo = 'bar';"),
            &PathBuf::from("test.tsx"),
        );

        assert!(file_path.exists());

        let file_contents = std::fs::read_to_string(file_path).unwrap();

        assert!(file_contents.starts_with("// Copyright © 2024 Navarrotech\n"));
        assert!(file_contents.contains("const foo = 'bar';"));
    }

    #[test]
    fn ensure_automatron_headers_rust() {
        let file_path = generate_temp_file(
            &String::from("let foo = String::from(\"noop\");"),
            &PathBuf::from("test.rs"),
        );

        assert!(file_path.exists());

        let file_contents = std::fs::read_to_string(file_path).unwrap();

        assert!(file_contents.starts_with("// Copyright © 2024 Navarrotech\n"));
        assert!(file_contents.contains("let foo = String::from(\"noop\");"));
    }
}

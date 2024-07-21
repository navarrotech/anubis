// Copyright © 2024 Navarrotech

use std::path::PathBuf;

use crate::cli::common::get_comment_type;
use crate::cli::common::get_copyright_header;
use crate::cli::common::get_file_type;
use crate::schema::AnubisSchema;

pub fn write_synthetic(schema: &AnubisSchema, content: &String, file_path: &PathBuf) {
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
  let preamble = format!("
{comment} This is a synthetic Anubis file. 
{comment} Synthetic are files that Anubis writes and manages, but Anubis will always honor your changes.
{comment} Take caution while editing this file, it may change in the future & you are in partial control of this file.
", comment = comment_type);

  if file_type != "md" && file_type != "html" && file_type != "json" {
    super_content = format!(
      "{copyright}\n{preamble}\n{content}",
      preamble = preamble,
      copyright = copyright,
      content = content
    );
  }

  let local_file_path = file_path.to_string_lossy().to_string().replace(
    &schema.install_directory.to_string_lossy().to_string(),
    ""
  );

  let anubis_analysis_file = schema.install_directory
    .clone()
    .join(".anubis/cache")
    .clone()
    .join(&local_file_path.clone()[1..]);

  // Past content is the last output from a synthetic file without user changes
  let mut past_content = String::from("");
  if anubis_analysis_file.exists() {
    past_content = std::fs::read_to_string(&anubis_analysis_file).unwrap();
  }

  let mut current_content = String::from("");
  if file_path.exists() {
    current_content = std::fs::read_to_string(file_path).unwrap();
  }

  let mut super_content_with_user_changes = super_content.clone();
  if !past_content.is_empty() && !current_content.is_empty() {
    // Compare each line of the past content with the super content
    // If the line is the same, that means there has been no user edits.
    // If the line is different, it could mean the following:
    // 1. The user added a line
    // 2. The user changed the line
    // 3. The user deleted the line
    // 4. This is old content and the the new content is supposed to add to it
    // 5. This is old content and the new content is supposed to replace it

    // TODO: Figure out how to handle respecting user deleted lines

    // let past_lines: Vec<&str> = past_content.lines().collect();
    // let current_lines: Vec<&str> = current_content.lines().collect();
    // let super_lines: Vec<&str> = super_content.lines().collect();

    // let mut merged_lines = vec![];
    // let mut past_index = 0;
    // let mut current_index = 0;
    // let mut super_index = 0;

    // while past_index < past_lines.len() && current_index < current_lines.len() {
    //     if past_lines[past_index] == current_lines[current_index] {
    //         merged_lines.push(super_lines[super_index].to_string());
    //         past_index += 1;
    //         current_index += 1;
    //         super_index += 1;
    //     } else if past_index < past_lines.len() {
    //         merged_lines.push(current_lines[current_index].to_string());
    //         current_index += 1;
    //     }
    // }

    // while current_index < current_lines.len() {
    //     merged_lines.push(current_lines[current_index].to_string());
    //     current_index += 1;
    // }

    // while super_index < super_lines.len() {
    //     merged_lines.push(super_lines[super_index].to_string());
    //     super_index += 1;
    // }

    // super_content_with_user_changes = merged_lines.join("\n");
}

  if !super_content_with_user_changes.ends_with('\n') {
    super_content_with_user_changes.push('\n');
  }

  // Write the content to the file
  std::fs::write(file_path, super_content_with_user_changes).expect(&format!(
      "Unable to write {file_name} file",
      file_name = file_name
  ));

  // After the core file is written, we re-write the past_content .anubis file with the non-user edited content

  // Write all parent directories in the path if they don't exist
  std::fs::create_dir_all(anubis_analysis_file.parent().unwrap()).expect(&format!(
      "Unable to create parent directories for {file_path}",
      file_path = anubis_analysis_file.display()
  ));

  std::fs::write(anubis_analysis_file, super_content).expect(&format!(
      "Unable to write {file_name} file",
      file_name = file_name
  ));
}



#[cfg(test)]
mod synthetics {
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

        write_synthetic(&test_schema, &content, &path_upgraded);

        path_upgraded
    }

    #[test]
    fn ensure_synthetics_headers_yaml() {
        let file_path = generate_temp_file(&String::from("Foo Bazz"), &PathBuf::from("test.yml"));

        assert!(file_path.exists());

        let file_contents = std::fs::read_to_string(file_path).unwrap();

        assert!(file_contents.starts_with("# Copyright © 2024 Navarrotech\n\n"));
        assert!(file_contents.contains("Foo Bazz"));
    }

    #[test]
    fn ensure_synthetics_headers_javascript() {
        let file_path = generate_temp_file(
            &String::from("const foo = 'bar';"),
            &PathBuf::from("test.js"),
        );

        assert!(file_path.exists());

        let file_contents = std::fs::read_to_string(file_path).unwrap();

        assert!(file_contents.starts_with("// Copyright © 2024 Navarrotech\n\n"));
        assert!(file_contents.contains("const foo = 'bar';"));
    }

    #[test]
    fn ensure_synthetics_headers_typescript() {
        let file_path = generate_temp_file(
            &String::from("const foo = 'bar';"),
            &PathBuf::from("test.ts"),
        );

        assert!(file_path.exists());

        let file_contents = std::fs::read_to_string(file_path).unwrap();

        assert!(file_contents.starts_with("// Copyright © 2024 Navarrotech\n\n"));
        assert!(file_contents.contains("const foo = 'bar';"));
    }

    #[test]
    fn ensure_synthetics_headers_typescript_react() {
        let file_path = generate_temp_file(
            &String::from("const foo = 'bar';"),
            &PathBuf::from("test.tsx"),
        );

        assert!(file_path.exists());

        let file_contents = std::fs::read_to_string(file_path).unwrap();

        assert!(file_contents.starts_with("// Copyright © 2024 Navarrotech\n\n"));
        assert!(file_contents.contains("const foo = 'bar';"));
    }

    #[test]
    fn ensure_synthetics_headers_rust() {
        let file_path = generate_temp_file(
            &String::from("let foo = String::from(\"noop\");"),
            &PathBuf::from("test.rs"),
        );

        assert!(file_path.exists());

        let file_contents = std::fs::read_to_string(file_path).unwrap();

        assert!(file_contents.starts_with("// Copyright © 2024 Navarrotech\n\n"));
        assert!(file_contents.contains("let foo = String::from(\"noop\");"));
    }

    #[test]
    fn first_time_synthetic_write() {
        let temp_directory = tempdir().unwrap().into_path();

        let mut test_schema = AnubisSchema::default();
        test_schema.project_name = "Anubis Test".to_string();
        test_schema.copyright_header = String::from("Copyright © {YYYY} Navarrotech");
        test_schema.copyright_header_formatted = String::from("Copyright © 2024 Navarrotech");
        test_schema.install_directory = temp_directory.clone();

        let file_path = temp_directory.clone().join("test.rs");

        let content = String::from("<!doctype html>
<html lang=\"en\">
  <head>
    <meta charset=\"UTF-8\" />
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />
    <meta content=\"text/html;charset=utf-8\" http-equiv=\"Content-Type\">

    <link rel=\"shortcut icon\" href=\"/logo.png\" type=\"image/x-icon\">
    <link rel=\"icon\" href=\"/logo.png\" type=\"image/x-icon\">
    
    <title>Test</title>
  </head>
  <body>
    <div id=\"root\"></div>
    <script type=\"module\" src=\"/src/main.tsx\"></script>
  </body>
</html>");

        write_synthetic(&test_schema, &content, &file_path);

        assert!(file_path.exists());

        let file_contents = std::fs::read_to_string(&file_path).unwrap();

        assert!(file_contents.starts_with("//"));
        assert!(file_contents.contains(&content));
    }

    #[test]
    fn synthetic_write_respect_user_addins() {
        let temp_directory = tempdir().unwrap().into_path();

        let mut test_schema = AnubisSchema::default();
        test_schema.project_name = "Anubis Test".to_string();
        test_schema.copyright_header = String::from("Copyright © {YYYY} Navarrotech");
        test_schema.copyright_header_formatted = String::from("Copyright © 2024 Navarrotech");
        test_schema.install_directory = temp_directory.clone();

        let file_path = temp_directory.clone().join("addon_test.md");

        // Original content is missing content-type tag and seo tags
        let original_content = String::from("
a
b
c
");

        // The user added a link tag and meta tags
        let user_modified_content = String::from("
a
b
b1
c
");

        let updated_content = String::from("
a
b
c
d
");

        let expected_result = String::from("
a
b
b1
c
d
");

        // Write the original content
        write_synthetic(&test_schema, &original_content, &file_path);

        assert!(file_path.exists());

        // Update with user content
        std::fs::write(&file_path, &user_modified_content).expect(
          "Unable to write addon_test.rs file in synthetic writing unit test"
        );

        // Update with new content
        write_synthetic(&test_schema, &updated_content, &file_path);

        let file_contents = std::fs::read_to_string(file_path).unwrap();

        assert_eq!(file_contents, expected_result);
    }

//     #[test]
//     fn synthetic_write_respect_user_addins_with_headers() {
//         let temp_directory = tempdir().unwrap().into_path();

//         let mut test_schema = AnubisSchema::default();
//         test_schema.project_name = "Anubis Test".to_string();
//         test_schema.copyright_header = String::from("Copyright © {YYYY} Navarrotech");
//         test_schema.copyright_header_formatted = String::from("Copyright © 2024 Navarrotech");
//         test_schema.install_directory = temp_directory.clone();

//         let file_path = temp_directory.clone().join("addon_test.rs");

//         // Original content is missing content-type tag and seo tags
//         let original_content = String::from("
// a
// b
// c
// ");

//         // The user added a link tag and meta tags
//         let user_modified_content = String::from("// Copyright © 2024 Navarrotech


// // This is a synthetic Anubis file. 
// // Synthetic are files that Anubis writes and manages, but Anubis will always honor your changes.
// // Take caution while editing this file, it may change in the future & you are in partial control of this file.

// a
// b
// b1
// c
// ");

//         let updated_content = String::from("
// a
// b
// c
// d
// ");

//         let expected_result = String::from("// Copyright © 2024 Navarrotech


// // This is a synthetic Anubis file. 
// // Synthetic are files that Anubis writes and manages, but Anubis will always honor your changes.
// // Take caution while editing this file, it may change in the future & you are in partial control of this file.

// a
// b
// b1
// c
// d
// ");

//         // Write the original content
//         write_synthetic(&test_schema, &original_content, &file_path);

//         assert!(file_path.exists());

//         // Update with user content
//         std::fs::write(&file_path, &user_modified_content).expect(
//           "Unable to write addon_test.rs file in synthetic writing unit test"
//         );

//         // Update with new content
//         write_synthetic(&test_schema, &updated_content, &file_path);

//         let file_contents = std::fs::read_to_string(file_path).unwrap();

//         assert_eq!(file_contents, expected_result);
//     }

//     #[test]
//     fn synthetic_write_respect_user_changes() {
//         let temp_directory = tempdir().unwrap().into_path();

//         let mut test_schema = AnubisSchema::default();
//         test_schema.project_name = "Anubis Test".to_string();
//         test_schema.copyright_header = String::from("Copyright © {YYYY} Navarrotech");
//         test_schema.copyright_header_formatted = String::from("Copyright © 2024 Navarrotech");
//         test_schema.install_directory = temp_directory.clone();

//         let file_path = temp_directory.clone().join("change_test.md");

//         // Original content is missing content-type tag and seo tags
//         let original_content = String::from("
// apples
// bananas
// cats
// ");

//         // The user added a link tag and meta tags
//         let user_modified_content = String::from("
// apples
// bats
// cats
// ");

//         let updated_content = String::from("
// apples
// bananas
// cats
// ");

//         let expected_result = String::from("
// apples
// bats
// cats
// ");

//         // Write the original content
//         write_synthetic(&test_schema, &original_content, &file_path);

//         assert!(file_path.exists());

//         // Update with user content
//         std::fs::write(&file_path, &user_modified_content).expect(
//           "Unable to write addon_test.rs file in synthetic writing unit test"
//         );

//         // Update with new content
//         write_synthetic(&test_schema, &updated_content, &file_path);

//         let file_contents = std::fs::read_to_string(file_path).unwrap();

//         assert_eq!(file_contents, expected_result);
//     }

  }

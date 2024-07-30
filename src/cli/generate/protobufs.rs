// Copyright © 2024 Navarrotech

use crate::automatrons::write::write_automatron;
use crate::relics::write::write_relic;
use crate::schema::AnubisSchema;
use crate::models::{FormatChoice, ModelKind, Models, RelationshipMode, UseOption};

pub fn generate_protobufs(schema: &AnubisSchema) {
    generate_common_protobuf(schema);
    generate_auth_protobuf(schema);
    generate_custom_protobuf(schema);
    generate_root_protobuf(schema);

    for model in schema.models.iter() {
        generate_model_protobuf(schema, model);
    }
}

fn generate_auth_protobuf(schema: &AnubisSchema) {
    let auth_protobuf = format!(
        r#"
syntax = "proto3";

package auth;

// A standardized user struct for authentication
message User {{
    string id = 1;
    optional string email = 2;
    optional string phone = 3;
    string first_name = 4;
    string last_name = 5;
    string password = 6;
    optional string avatar = 7;
    optional string bio = 8;
    UserPreferences preferences = 9;
    string created_at = 10;
    string updated_at = 11;
}}

enum Theme {{
    LIGHT = 0;
    DARK = 1;
    SYSTEM = 2;
}}

// Preferences for the user
message UserPreferences {{
    string language = 1;
    Theme theme = 2;
    string timezone = 3;
}}

message AuthResponse {{
    bool authorized = 1;
    User user = 2;
}}

message UnauthorizedRequest {{
    bool authorized = 1;
}}

message AuthorizeByPhoneRequest {{
    string phone = 1;
    optional string OTP = 2;
}}
"#
    );

    write_automatron(
        schema,
        &auth_protobuf,
        &schema.install_directory.join("./proto/auth.proto"),
    );
}

fn generate_common_protobuf(schema: &AnubisSchema) {
    let common_protobuf = format!(
        r#"
syntax = "proto3";

package common;

// Used for reporting server-side errors to the client
// A code is used instead of a stringified message for internationalization
message ServerError {{
    int32 code = 1;
}}

// A blank message, sometimes used for signaling
message Blank {{
    int32 i = 1;
}}

// Get request enums
enum SortOrder {{
    ASC = 0;
    DESC = 1;
}}

enum FilterOperator {{
    EQUALS = 0;
    NOT_EQUALS = 1;
    GREATER_THAN = 2;
    LESS_THAN = 3;
    GREATER_THAN_OR_EQUAL = 4;
    LESS_THAN_OR_EQUAL = 5;
    CONTAINS = 6;
    NOT_CONTAINS = 7;
    STARTS_WITH = 8;
    ENDS_WITH = 9;
}}

// For GET requests to specify list vs single item
// Also handles pagination, search, etc
message ListRequest {{
    string id = 1;
    optional int32 skip = 2;
    optional int32 take = 3;
    optional string search = 4;
    optional string sort_by = 5;
    optional SortOrder sort_order = 6;
    repeated Filter filters = 7;
}}

// For GET requests to filter results
message Filter {{
    string field = 1;
    string value = 2;
    FilterOperator operator = 3;
}}

// Used for reporting client-side errors to the backend
message ClientErrorReport {{
    string message = 1;
    string stack_trace = 2;
}}

// Form validation errors to ensure the client is sending the correct data
// 1. Path is the field path (i.e. 'user.email')
// 2. Key is the field name (i.e. 'email')
// 3. Code is the error code for internationalization
message FormInvalid {{
    string path = 1;
    string key = 2;
    int32 code = 3;
    optional int32 max_length = 4;
    optional int32 min_length = 5;
    optional int32 max_value = 6;
    optional int32 min_value = 7;
    repeated string required_missing_fields = 8;
    optional bool invalid_type = 9;
    optional bool invalid_email = 10;
    optional bool invalid_phone = 11;
    
}}

message FormsInvalid {{
    repeated FormInvalid invalid = 1;
}}

// This is used for the client to specify a specific item from the server
// This is mostly used for DELETE requests or GET requests for a single item
message SpecifyRequest {{
    string id = 1;
}}
"#
    );

    write_automatron(
        schema,
        &common_protobuf,
        &schema.install_directory.join("./proto/common.proto"),
    );
}

fn generate_custom_protobuf(schema: &AnubisSchema) {
    let custom_protobuf = format!(
        r#"
syntax = "proto3";

package custom;

// Add and import your own custom protobuf structs here

"#);

    write_relic(
        schema,
        &custom_protobuf,
        &schema.install_directory.join("./proto/custom.proto"),
    );
}

fn generate_root_protobuf(schema: &AnubisSchema) {
    let models_imports = schema
        .models
        .iter()
        .map(|model| format!("import \"structs/{}.proto\";", model.name))
        .collect::<Vec<String>>()
        .join("\n");

    let mut models_sync = String::new();
    let mut models_changes = String::new();
    for (index, model) in schema.models.iter().enumerate() {
        models_sync.push_str(
            &format!(
                "    {} {} {} = {};\n",
                if model.mode == RelationshipMode::OneToMany { "repeated" } else { "optional" },
                model.name,
                model.name.to_lowercase(),
                index + 3
            )
        );

        models_changes.push_str(
            &format!(
                "    optional {} {} = {};\n",
                model.name,
                model.name.to_lowercase(),
                index + 2
            )
        );
    }

    let common_protobuf: String = format!(
        r#"
syntax = "proto3";

import "common.proto";
import "auth.proto";
import "custom.proto";

{models_imports}

// When the application initializes, it will send this message from the server
// This is designed to bring the client state up to speed with the server
message SyncResponse {{
    User user = 1;
    UserPreferences = 2;

{models_sync}
}}

// When a database item is updated that is relevant to the client, 
// the server will send this message to the client to keep their state in sync
message ChangeEvent {{
    string type = 1;

{models_changes}
}}
"#, models_imports = models_imports);

    write_automatron(
        schema,
        &common_protobuf,
        &schema.install_directory.join("./proto/root.proto"),
    );
}

fn generate_model_protobuf(schema: &AnubisSchema, model: &Models) {
    let mut inner_struct = String::new();
    let mut create_struct = String::new();
    let mut update_struct = String::new();

    // For each field in model.fields
    for (i, field) in model.fields.iter().enumerate() {
        let proto_type = match field.kind {
            ModelKind::String => "string",
            ModelKind::DateTime => "string",
            ModelKind::Number => "i32",
            ModelKind::Float => "f32",
            ModelKind::Boolean => "bool",
        };

        // We don't send passwords to the frontend!
        if field.format != Some(FormatChoice::Secret)
            && field.format != Some(FormatChoice::Password)
            && field.replicate == false
        {
            inner_struct.push_str(&format!(
                "    {} {} = {};\n",
                proto_type,
                field.name,
                i + 1
            ));
        }

        if field.name != "id"
            && field.use_method != Some(UseOption::CreatedAt)
            && field.use_method != Some(UseOption::UpdatedAt)
            && field.use_method != Some(UseOption::Uuid)
        {
            create_struct.push_str(&format!(
                "    {} {} {} = {};\n",
                if field.required { "" } else { "optional" },
                proto_type,
                field.name,
                i + 1
            ));
        }

        update_struct.push_str(&format!(
            "    {} {} {} = {};\n",
            if field.required || field.name == "id" { "" } else { "optional" },
            proto_type,
            field.name,
            i + 1
        ));
    }

    let model_protobuf = format!(
        r#"
syntax = "proto3";

package structs;

message {name} {{
{inner_struct}
}}

message create_{name} {{
{create_struct}
}}

message update_{name} {{
{update_struct}
}}
"#,
    name = model.name,
    inner_struct = inner_struct,
    create_struct = create_struct,
    update_struct = update_struct
);

    write_automatron(
        schema,
        &model_protobuf,
        &schema.install_directory.join(
            format!("./proto/structs/{}.proto", model_protobuf)
        ),
    );
}

// Copyright © 2024 Navarrotech

use std::path::PathBuf;

use crate::automatrons::write::write_automatron;
use crate::schema::AnubisSchema;

pub fn generate_protobufs(schema: &AnubisSchema) {}

pub fn generate_common_protobuf(schema: &AnubisSchema) {
    let common_protobuf = format!(
        r#"
syntax = "proto3";

package common;

message Error {{
    string message = 1;
    string code = 2;
}}
"#
    );

    write_automatron(
        schema,
        &common_protobuf,
        &PathBuf::from("./proto/common.proto"),
    );
}

pub fn generate_root_protobuf(schema: &AnubisSchema) {
    let common_protobuf: String = format!(
        r#"
syntax = "proto3";

import "common.proto";

"#
    );

    write_automatron(
        schema,
        &common_protobuf,
        &PathBuf::from("./proto/common.proto"),
    );
}

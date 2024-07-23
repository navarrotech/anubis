// Copyright © 2024 Navarrotech

pub fn create_tsconfig() -> String {
    String::from(
        "{
  \"compilerOptions\": {
    \"target\": \"ES2020\",
    \"useDefineForClassFields\": true,
    \"module\": \"ESNext\",
    \"lib\": [\"ESNext\", \"DOM\", \"DOM.Iterable\"],
    \"skipLibCheck\": true,

    /* Bundler mode */
    \"moduleResolution\": \"bundler\",
    \"allowImportingTsExtensions\": true,
    \"resolveJsonModule\": true,
    \"isolatedModules\": true,
    \"noEmit\": true,
    \"jsx\": \"react-jsx\",
    \"jsxImportSource\": \"react\",

    /* Linting */
    \"strict\": true,
    \"noUnusedLocals\": true,
    \"noUnusedParameters\": true,
    \"noFallthroughCasesInSwitch\": true,

    /* Pathing */
    \"baseUrl\": \"./src\",
    \"paths\": {
      \"@/*\": [\"*\"]
    }
  },
  \"include\": [
    \"src\"
  ],
  \"exclude\": [
    \"node_modules\",
    \"dist\"
  ],
  \"references\": [
    { \"path\": \"./tsconfig.node.json\" }
  ]
}
",
    )
}

pub fn create_tsconfig_node() -> String {
    String::from(
        "{
  \"compilerOptions\": {
    \"composite\": true,
    \"skipLibCheck\": true,
    \"module\": \"ESNext\",
    \"moduleResolution\": \"bundler\",
    \"allowSyntheticDefaultImports\": true,
    \"strict\": true
  },
  \"include\": [\"vite.config.ts\"]
}
",
    )
}

#[cfg(test)]
mod check_ts_configs {
    use super::*;
    use json;

    fn strip_json_comments(json: &str) -> String {
        let mut stripped = String::new();

        // For each line in json string...
        // If trimmed line does not starts with "//" or "/*" then add to stripped
        let lines = json.lines();
        for line in lines {
            let trimmed = line.trim();
            if !trimmed.starts_with("//") && !trimmed.starts_with("/*") {
                stripped.push_str(line);
            }
        }
        stripped
    }

    #[test]
    fn ensure_tsconfig_json_is_valid() {
        let content = create_tsconfig();

        let stripped = strip_json_comments(&content);
        let parsed = json::parse(&stripped);

        assert!(parsed.is_ok());
    }

    #[test]
    fn ensure_tsconfig_node_json_is_valid() {
        let content = create_tsconfig_node();
        let stripped = strip_json_comments(&content);
        let parsed = json::parse(&stripped);

        assert!(parsed.is_ok());
    }
}

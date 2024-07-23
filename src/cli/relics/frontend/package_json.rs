// Copyright © 2024 Navarrotech

use crate::schema::AnubisSchema;

pub fn create_package_json(schema: &AnubisSchema) -> String {
    let project_name = schema.project_name.replace(' ', "-");

    format!(
        "{{
  \"name\": \"{project_name}\",
  \"private\": true,
  \"version\": \"1.0.0\",
  \"type\": \"module\",
  \"scripts\": {{
    \"dev\": \"yarn install && vite\",
    \"build\": \"tsc && vite build\",
    \"preview\": \"vite preview\",
    \"analyze\": \"vite-bundle-visualizer\",
    \"test\": \"vitest run --dom --coverage src/\",
    \"test:dev\": \"DEBUG_PRINT_LIMIT=100000 vitest watch --ui --dom --coverage.enabled=true\",
    \"lint\": \"eslint . --ext ts,tsx --report-unused-disable-directives\",
    \"lint:watch\": \"nodemon --exec 'yarn lint' --watch src --watch .eslintrc.js\"
  }},
  \"engines\": {{
    \"node\": \">=20.11.0\",
    \"yarn\": \">=1.22.0\"
  }},
  \"dependencies\": {{
    \"@fortawesome/fontawesome-svg-core\": \"^6.5.2\",
    \"@fortawesome/free-regular-svg-icons\": \"^6.5.2\",
    \"@fortawesome/free-solid-svg-icons\": \"^6.5.2\",
    \"@fortawesome/react-fontawesome\": \"^0.2.2\",
    \"axios\": \"^1.7.2\",
    \"i18next\": \"^23.11.3\",
    \"i18next-browser-languagedetector\": \"^8.0.0\",
    \"i18next-http-backend\": \"^2.5.1\",
    \"js-logger\": \"^1.6.1\",
    \"lodash-es\": \"^4.17.21\",
    \"moment\": \"^2.30.1\",
    \"protobufjs\": \"^7.3.0\",
    \"react\": \"^18.3.1\",
    \"react-browser-router\": \"^2.1.2\",
    \"react-dom\": \"^18.3.1\",
    \"react-i18next\": \"^14.1.1\",
    \"react-icons\": \"^5.2.1\",
    \"react-router\": \"^6.23.1\",
    \"react-router-dom\": \"^6.23.1\",
    \"reconnecting-websocket\": \"^4.4.0\",
    \"sass\": \"^1.77.6\",
    \"spiccato\": \"^1.0.0-beta\",
    \"spiccato-react\": \"1.0.1-beta\",
    \"yup\": \"^1.4.0\"
  }},
  \"devDependencies\": {{
    \"@stylistic/eslint-plugin-js\": \"^2.2.1\",
    \"@testing-library/react\": \"^16.0.0\",
    \"@types/lodash-es\": \"^4.17.12\",
    \"@types/node\": \"^20.14.5\",
    \"@types/react\": \"^18.3.3\",
    \"@types/react-dom\": \"^18.3.0\",
    \"@typescript-eslint/eslint-plugin\": \"^7.2.0\",
    \"@typescript-eslint/parser\": \"^7.2.0\",
    \"@vitejs/plugin-react-swc\": \"^3.7.0\",
    \"@vitest/coverage-v8\": \"^1.6.0\",
    \"@vitest/ui\": \"^1.6.0\",
    \"autoprefixer\": \"^10.4.19\",
    \"eslint\": \"^8.57.0\",
    \"eslint-config-airbnb\": \"^19.0.4\",
    \"eslint-config-google\": \"^0.14.0\",
    \"eslint-import-resolver-typescript\": \"^3.6.1\",
    \"eslint-plugin-header\": \"^3.1.1\",
    \"eslint-plugin-i18next\": \"^6.0.3\",
    \"eslint-plugin-import\": \"^2.29.1\",
    \"eslint-plugin-jsx-a11y\": \"^6.9.0\",
    \"eslint-plugin-react\": \"^7.34.3\",
    \"eslint-plugin-react-hooks\": \"^4.6.0\",
    \"eslint-plugin-react-refresh\": \"^0.4.6\",
    \"happy-dom\": \"^14.12.0\",
    \"nodemon\": \"^3.1.4\",
    \"postcss\": \"^8.4.39\",
    \"prettier\": \"3.3.2\",
    \"tailwindcss\": \"^3.4.6\",
    \"typescript\": \"^5.2.2\",
    \"vite\": \"^5.2.0\",
    \"vite-bundle-visualizer\": \"1.1.0\",
    \"vite-plugin-cesium\": \"^1.2.22\",
    \"vite-plugin-svgr\": \"^4.2.0\",
    \"vite-tsconfig-paths\": \"^4.3.2\",
    \"vitest\": \"^1.6.0\"
  }}
}}",
        project_name = project_name,
    )
}

#[cfg(test)]
mod check_package_json {
    use super::*;
    use crate::schema::AnubisSchema;
    use json;

    #[test]
    fn ensure_json_is_valid() {
        let mut test_schema = AnubisSchema::default();
        test_schema.project_name = "test".to_string();

        let content = create_package_json(&test_schema);
        let parsed = json::parse(content.as_str());

        assert!(parsed.is_ok());
    }

    #[test]
    fn ensure_json_is_valid_with_project_name_spaces() {
        let mut test_schema = AnubisSchema::default();
        test_schema.project_name = "name with spaces".to_string();

        let content = create_package_json(&test_schema);
        let parsed = json::parse(content.as_str());

        assert!(parsed.is_ok());
    }
}

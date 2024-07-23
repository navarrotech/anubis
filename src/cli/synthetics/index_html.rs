// Copyright © 2024 Navarrotech

use crate::schema::AnubisSchema;

pub fn create_frontend_html(schema: &AnubisSchema) -> String {
    format!(
        "<!doctype html>
<html lang=\"en\">
  <head>
    <meta charset=\"UTF-8\" />
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />
    <meta content=\"text/html;charset=utf-8\" http-equiv=\"Content-Type\">

    <link rel=\"shortcut icon\" href=\"/logo.png\" type=\"image/x-icon\">
    <link rel=\"icon\" href=\"/logo.png\" type=\"image/x-icon\">

    <title>{project_name}</title>
    <meta name=\"description\" content=\"{description}\">

    <!-- Open Graph / Facebook -->
    <meta property=\"og:title\" content=\"{project_name}\">
    <meta property=\"og:description\" content=\"{description}\">
    <meta property=\"og:image\" content=\"\">
    <meta property=\"og:url\" content=\"\">
    <meta property=\"og:type\" content=\"\">

    <!-- Twitter -->
    <meta name=\"twitter:card\" content=\"\">
    <meta name=\"twitter:title\" content=\"{project_name}\">
    <meta name=\"twitter:description\" content=\"{description}\">
    <meta name=\"twitter:image\" content=\"\">

    <!-- Additional SEO -->
    <meta name=\"robots\" content=\"\">
    <meta name=\"keywords\" content=\"\">
  </head>
  <body>
    <div id=\"root\"></div>
    <script type=\"module\" src=\"/src/main.tsx\"></script>
  </body>
</html>
",
        project_name = schema.project_name,
        description = schema.description
    )
}

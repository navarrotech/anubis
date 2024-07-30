// Copyright © 2024 Navarrotech

use crate::relics::write::write_relic;
use crate::schema::AnubisSchema;

pub fn generate_scss(schema: &AnubisSchema) {
  write_relic(
      schema,
      &format!(r#"
// Sass modules
@import "@/sass/bulma.scss"
@import "@/sass/fonts.sass"
"#),
      &schema.install_directory.join("frontend/src/index.sass"),
  );
  
  write_relic(
    schema,
    &format!(r#"
// Branding
$primary: #48d0fd
$link: #48d0fd
$info: #7934c5
$danger: #ff3860
$warning: #ffdd57
$success: #15af3e
$discord: #5865F2

$topbarHeight: 3.3em
$bottombarHeight: 4em

$black: #000000
$darker: #1A1A1C
$dark: #2C2C30
$darkish: #434249

$white: #FFFFFF
$lighter: #F8F8F8
$light: #F0F0F0
$lightish: #D3D3DE

$gap: 32px
$tablet: 769px
$desktop: 960px + 2 * $gap
$widescreen: 1152px + 2 * $gap
$fullhd: 1344px + 2 * $gap
"#),
    &schema.install_directory.join("frontend/src/sass/theme.sass"),
  );

  write_relic(
    schema,
    &format!(
r#"// https://fonts.google.com/share?selection.family=Montserrat:ital,wght@0,100..900;1,100..900
@import url('https://fonts.googleapis.com/css2?family=Montserrat:ital,wght@0,100..900;1,100..900&display=swap')

// https://fonts.google.com/share?selection.family=Archivo+Black
@import url('https://fonts.googleapis.com/css2?family=Archivo+Black&display=swap')

"#),
    &schema.install_directory.join("frontend/src/sass/fonts.sass"),
  );

  write_relic(
    schema,
    &format!(r#"
$family-titles: "Archivo Black", Roboto, sans-serif;
$family-primary: "Montserrat", Arial, sans-serif;

@use "bulma/sass" with (
  $family-primary: $family-primary,
  $primary: $primary,
  $link: $link,
  $info: $info,
  $title-family: $family-titles,
  $input-shadow: none
);

// Bulma extensions
@import "bulma-divider";
"#),
    &schema.install_directory.join("frontend/src/sass/fonts.sass"),
  );
}

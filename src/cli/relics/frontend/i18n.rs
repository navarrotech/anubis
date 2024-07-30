// Copyright © 2024 Navarrotech

use crate::relics::write::write_relic;
use crate::schema::AnubisSchema;

pub fn generate_i18next(schema: &AnubisSchema) {
    let auth_protobuf = format!(
        r#"
import i18next from 'i18next'
import {{ initReactI18next }} from 'react-i18next'
import Backend from 'i18next-http-backend'
import LanguageDetector from 'i18next-browser-languagedetector'

// Edit the default language here:
export const defaultLanguage = 'en' as const

// The core i18next instance that will be used throughout the app to manage language
export const i18Instance = i18next
  .use(initReactI18next)
  .use(LanguageDetector)
  .use(Backend)
  .init({{
    ns: [ 'translation' ],
    defaultNS: 'translation',
    lng: defaultLanguage,
    fallbackLng: defaultLanguage
  }})

// Add more languages here as you support them
export const supportedLanguages = [
  'en'
] as const

export const languageToFlag: Record<LanguageKey, string> = {{
  'en': 'US',
  'es': 'ES',
  'fr': 'FR',
  'ja': 'JP'
}} as const

// Add/remove from this list if needed
// It serves as a base of all standard languages
export const languageLocalizedRecord: Record<string, string> = {{
  en: 'English',
  zh: '中文',
  es: 'Español',
  ar: 'العربية',
  hi: 'हिन्दी',
  fr: 'Français',
  ru: 'Русский',
  pt: 'Português',
  de: 'Deutsch',
  ja: '日本語',
  ko: '한국어',
  vi: 'Tiếng Việt',
  it: 'Italiano',
  tr: 'Türkçe',
  pl: 'Polski',
  uk: 'Українська',
  nl: 'Nederlands',
  th: 'ไทย',
  sv: 'Svenska',
  da: 'Dansk',
  fi: 'Suomi',
  no: 'Norsk',
  he: 'עברית',
  el: 'Ελληνικά',
  hu: 'Magyar',
  cs: 'Čeština',
  ro: 'Română',
  bg: 'Български',
  sk: 'Slovenčina',
  lt: 'Lietuvių',
  lv: 'Latviešu',
  et: 'Eesti',
  hr: 'Hrvatski',
  sl: 'Slovenščina',
  sr: 'Српски',
  mk: 'Македонски',
  bs: 'Bosanski',
  mt: 'Malti',
  is: 'Íslenska',
  ga: 'Gaeilge',
  cy: 'Cymraeg',
  be: 'Беларуская',
  hy: 'Հայերեն',
  ka: 'ქართული',
  az: 'Azərbaycan',
  eu: 'Euskara',
  ca: 'Català',
  gl: 'Galego',
  sq: 'Shqip',
  mn: 'Монгол',
  ur: 'اردو',
  fa: 'فارسی',
  ta: 'தமிழ்',
  te: 'తెలుగు',
  ml: 'മലയാളം',
  kn: 'ಕನ್ನಡ',
  mr: 'मराठी',
  gu: 'ગુજરાતી',
  pa: 'ਪੰਜਾਬੀ',
  bn: 'বাংলা',
  my: 'မြန်မာဘာသာ',
  km: 'ខ្មែរ',
  lo: 'ລາວ',
  si: 'සිංහල'
}} as const

// Usable types
export type SupportedLanguages = typeof supportedLanguages[number]
export type LanguageKey = keyof typeof languageLocalizedRecord
"#
    );

    write_relic(
        schema,
        &auth_protobuf,
        &schema
            .install_directory
            .join("frontend/src/modules/i18n.ts"),
    );
}

pub fn generate_translation_json(schema: &AnubisSchema) {
    let auth_protobuf = format!(
        r#"
{{
  "brand_name": "{project_name}",
  "brand_subtitle": "Your brand subtitle here",
  
  "terms_agreement": "By signing in, you are agreeing to our <0>Terms</0> and <1>Privacy Policy</1>.",
  "terms_of_service": "Terms",
  "privacy_policy": "Privacy Policy",
  
  "sign_in_with_phone": "Sign in with a phone number",
  "sign_in_with_discord": "Sign in with Discord",
  "sign_in_with_google": "Sign in with Google",

  "enter_phone": "Enter your phone number",
  "phone_help": "A code will be sent to verify that it's really you. Message and data rates may apply.",
  "phone_changed": "Click here if you changed your phone number.",
  "enter_code": "Enter the code sent to your phone",
  "resend_code_wait": "You can resend the code in {{seconds}} seconds",
  "resend_code": "Didn't receive it? Click here to resend the code",

  "something_went_wrong": "Something went wrong.",
  "failed_to_connect_to_api": "Failed to connect to the server. Automatically retrying...",

  "generic_error":  "Something went wrong. Please try again.",
  "choose_language": "Select your language"
}}
"#,
        project_name = schema.project_name
    );

    write_relic(
        schema,
        &auth_protobuf,
        &schema
            .install_directory
            .join("frontend/public/locales/en/translation.json"),
    );
}

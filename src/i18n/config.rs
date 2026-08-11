use dioxus_i18n::prelude::*;
use dioxus_i18n::unic_langid::{LanguageIdentifier, langid};

pub fn config(initial_language: LanguageIdentifier) -> I18nConfig {
    I18nConfig::new(initial_language)
        .with_locale((langid!("en-GB"), include_str!("../../i18n/en/en-GB.ftl")))
        .with_locale((langid!("en"), include_str!("../../i18n/en/en-GB.ftl")))
        .with_fallback(langid!("en-GB"))
}

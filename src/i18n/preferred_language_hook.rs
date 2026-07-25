use dioxus::prelude::*;
use web_sys;

use crate::i18n::Language;

pub fn use_preferred_language() -> Signal<Language> {
    use std::str::FromStr;

    use_signal(|| {
        web_sys::window()
            .and_then(|w| w.navigator().language())
            .and_then(|l| Language::from_str(&l).ok())
            .unwrap_or_else(Language::default)
    })
}

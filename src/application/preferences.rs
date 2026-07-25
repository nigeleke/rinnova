use serde::{Deserialize, Serialize};

use crate::application::setup::Setup;
use crate::i18n::Language;

#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Preferences {
    setup: Setup,
    language: Option<Language>,
}

impl Preferences {
    pub fn language(&self) -> Option<&Language> {
        self.language.as_ref()
    }

    pub fn set_language(&mut self, language: Language) {
        self.language = Some(language)
    }

    pub fn setup(&self) -> Setup {
        self.setup
    }

    pub fn mark_welcome_seen(&mut self) {
        self.setup = Setup::Terms
    }

    pub fn mark_terms_seen(&mut self) {
        self.setup = Setup::Complete
    }
}

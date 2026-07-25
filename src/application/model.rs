use delegate::delegate;
use serde::{Deserialize, Serialize};

use crate::application::preferences::Preferences;
use crate::application::setup::Setup;
use crate::application::ui_state::{UiState, View};
use crate::i18n::Language;

#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Model {
    preferences: Preferences,
    ui_state: UiState,
}

impl Model {
    delegate! {
        to self.preferences {
            pub fn language(&self) -> Option<&Language>;
            pub fn set_language(&mut self, language: Language);
            pub fn setup(&self) -> Setup;
            pub fn mark_welcome_seen(&mut self);
            pub fn mark_terms_seen(&mut self);
        }

        to self.ui_state {
            pub fn view(&self) -> View;
            pub fn set_view(&mut self, view: View);
        }
    }
}

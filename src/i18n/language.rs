use dioxus_i18n::unic_langid::{LanguageIdentifier, LanguageIdentifierError};
use serde::{Deserialize, Serialize};
use thiserror::*;

#[derive(Debug, Error)]
pub enum LanguageError {
    #[error("invalid language identifier")]
    UnknownLanguage(#[from] LanguageIdentifierError),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Language(String);

impl Language {
    pub fn identifier(&self) -> LanguageIdentifier {
        LanguageIdentifier::from_bytes(self.0.as_bytes())
            .expect("validated language identifier string")
    }
}

impl Default for Language {
    fn default() -> Self {
        Self("en-GB".into())
    }
}

impl std::str::FromStr for Language {
    type Err = LanguageError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        LanguageIdentifier::from_bytes(s.as_bytes())?;
        Ok(Self(String::from(s)))
    }
}

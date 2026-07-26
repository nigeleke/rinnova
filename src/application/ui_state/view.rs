use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum View {
    #[default]
    HomePage,
    Reminders,
    Refills,
    Scripts,
    Medications,
}

impl std::fmt::Display for View {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            View::HomePage => "home-page",
            View::Reminders => "reminders",
            View::Refills => "refills",
            View::Scripts => "scripts",
            View::Medications => "medications",
        }
        .fmt(f)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ConfirmationTheme {
    Info,
    Warning,
    Destructive,
    Error,
}

impl std::fmt::Display for ConfirmationTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Info => "confirmation-info",
            Self::Warning => "confirmation-warning",
            Self::Destructive => "confirmation-destructive",
            Self::Error => "confirmation-error",
        }
        .fmt(f)
    }
}

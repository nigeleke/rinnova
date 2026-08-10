#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ConfirmationTheme {
    _Info,
    _Warning,
    Destructive,
    _Error,
}

impl std::fmt::Display for ConfirmationTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::_Info => "confirmation-info",
            Self::_Warning => "confirmation-warning",
            Self::Destructive => "confirmation-destructive",
            Self::_Error => "confirmation-error",
        }
        .fmt(f)
    }
}

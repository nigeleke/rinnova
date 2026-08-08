#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Health {
    Ok,
    Attention,
    Critical,
}

impl std::fmt::Display for Health {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok => "health-ok",
            Self::Attention => "health-attention",
            Self::Critical => "health-critical",
        }
        .fmt(f)
    }
}

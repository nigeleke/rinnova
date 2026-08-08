#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NotificationLevel {
    Info,
    Success,
    Warning,
    Error,
}

impl NotificationLevel {
    pub fn class(&self) -> &'static str {
        match self {
            NotificationLevel::Info => "notification-info",
            NotificationLevel::Success => "notification-success",
            NotificationLevel::Warning => "notification-warning",
            NotificationLevel::Error => "notification-error",
        }
    }
}

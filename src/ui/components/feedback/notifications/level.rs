#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NotificationLevel {
    _Info,
    _Success,
    Warning,
    Error,
}

impl NotificationLevel {
    pub fn class(&self) -> &'static str {
        match self {
            NotificationLevel::_Info => "notification-info",
            NotificationLevel::_Success => "notification-success",
            NotificationLevel::Warning => "notification-warning",
            NotificationLevel::Error => "notification-error",
        }
    }
}

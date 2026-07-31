use super::level::NotificationLevel;

#[derive(Clone)]
pub struct Notification {
    level: NotificationLevel,
    message: String,
}

impl Notification {
    pub fn info(message: &str) -> Self {
        Self::new(NotificationLevel::Info, message)
    }

    pub fn success(message: &str) -> Self {
        Self::new(NotificationLevel::Success, message)
    }

    pub fn warning(message: &str) -> Self {
        Self::new(NotificationLevel::Warning, message)
    }

    pub fn error(message: &str) -> Self {
        Self::new(NotificationLevel::Error, message)
    }

    fn new(level: NotificationLevel, message: &str) -> Self {
        Self {
            level,
            message: message.into(),
        }
    }
}

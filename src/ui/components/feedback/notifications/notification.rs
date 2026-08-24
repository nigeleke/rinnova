use dioxus::prelude::*;
use dioxus_i18n::tid;
use gloo_timers::future::TimeoutFuture;

use crate::domain::LogbookError;
use crate::storage::StorageError;
use crate::ui::components::{NotificationId, NotificationLevel};

#[derive(Clone, PartialEq, Eq)]
pub struct Notification {
    id: NotificationId,
    level: NotificationLevel,
    message: String,
}

impl Notification {
    pub fn id(&self) -> NotificationId {
        self.id
    }

    pub fn info(message: &str) -> Self {
        Self::new(NotificationLevel::Info, message)
    }

    pub fn _success(message: &str) -> Self {
        Self::new(NotificationLevel::_Success, message)
    }

    pub fn warning(message: &str) -> Self {
        Self::new(NotificationLevel::Warning, message)
    }

    pub fn error(message: &str) -> Self {
        Self::new(NotificationLevel::Error, message)
    }

    fn new(level: NotificationLevel, message: &str) -> Self {
        let id = NotificationId::new();

        Self {
            id,
            level,
            message: message.into(),
        }
    }

    pub fn class(&self) -> &'static str {
        self.level.class()
    }

    pub fn message(key: &str) {
        let notification = Notification::info(&tid!(key));
        add_notification(notification);
    }

    pub fn storage_error(error: &StorageError) {
        let notification = match error {
            StorageError::IndexedDb => {
                let error = tid!("error.internal-error", error: error.to_string());
                Notification::error(&error)
            }

            StorageError::Serde => {
                let error = tid!("error.internal-error", error: error.to_string());
                Notification::error(&error)
            }
        };

        add_notification(notification);
    }

    pub fn logbook_error(error: &LogbookError) {
        let i18n_key = error.to_string();

        let notification = match error {
            LogbookError::InvalidDate => {
                let error = tid!(&i18n_key);
                Notification::error(&error)
            }

            LogbookError::InvalidDateRange => {
                let error = tid!(&i18n_key);
                Notification::warning(&error)
            }

            LogbookError::MatchingMedication(value) => {
                let error = tid!(&i18n_key, name: value);
                Notification::warning(&error)
            }

            LogbookError::DuplicateMedication(id) => {
                let error = tid!(&i18n_key, id: id.to_string());
                Notification::error(&error)
            }

            LogbookError::InvalidMedication(id) => {
                let error = tid!(&i18n_key, id: id.to_string());
                Notification::error(&error)
            }

            LogbookError::MedicationUsedInScript => {
                let error = tid!(&i18n_key);
                Notification::warning(&error)
            }

            LogbookError::NoMedications => {
                let error = tid!(&i18n_key);
                Notification::warning(&error)
            }

            LogbookError::DuplicateScript(id) => {
                let error = tid!(&i18n_key, id: id.to_string());
                Notification::error(&error)
            }

            LogbookError::InvalidScript(id) => {
                let error = tid!(&i18n_key, id: id.to_string());
                Notification::error(&error)
            }

            LogbookError::ScriptUsedInSupply => {
                let error = tid!(&i18n_key);
                Notification::warning(&error)
            }

            LogbookError::UnknownMedication(id) => {
                let error = tid!(&i18n_key, id: id.to_string());
                Notification::error(&error)
            }

            LogbookError::DuplicateSupply(id) => {
                let error = tid!(&i18n_key, id: id.to_string());
                Notification::error(&error)
            }

            LogbookError::InvalidSupply(id) => {
                let error = tid!(&i18n_key, id: id.to_string());
                Notification::error(&error)
            }

            LogbookError::ScriptOutOfDate(id) => {
                let error = tid!(&i18n_key, id: id.to_string());
                Notification::error(&error)
            }

            LogbookError::SupplyHasNoMedications => {
                let error = tid!(&i18n_key);
                Notification::warning(&error)
            }

            LogbookError::SupplyHasDuplicateMedications => {
                let error = tid!(&i18n_key);
                Notification::warning(&error)
            }

            LogbookError::MedicationNotOnScript(script_id, medication_id) => {
                let error = tid!(&i18n_key, script_id: script_id.to_string(), medication_id: medication_id.to_string());
                Notification::error(&error)
            }
        };

        add_notification(notification);
    }
}

impl std::fmt::Display for Notification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(f)
    }
}

fn add_notification(notification: Notification) {
    let mut notifications = use_context::<Signal<Vec<Notification>>>();

    let id = notification.id();
    notifications.write().push(notification);

    spawn(async move {
        TimeoutFuture::new(3000).await;
        notifications.write().retain(|n| n.id() != id);
    });
}

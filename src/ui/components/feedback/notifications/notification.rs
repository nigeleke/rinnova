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

    pub fn _info(message: &str) -> Self {
        Self::new(NotificationLevel::_Info, message)
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
        let notification = match error {
            LogbookError::InvalidDate => {
                let error = tid!("error.invalid-date");
                Notification::error(&error)
            }

            LogbookError::MatchingMedication(value) => {
                let error = tid!("error.matching-medication", name: value);
                Notification::warning(&error)
            }

            LogbookError::DuplicateMedication(id) => {
                let error = tid!("error.duplicate-medication", id: id.to_string());
                Notification::error(&error)
            }

            LogbookError::InvalidMedication(id) => {
                let error = tid!("error.invalid-medication", id: id.to_string());
                Notification::error(&error)
            }

            LogbookError::InvalidDraftMedication => {
                let error = tid!("error.invalid-draft-medication");
                Notification::error(&error)
            }

            LogbookError::MedicationUsedInScript => {
                let error = tid!("error.medication-used-in-script");
                Notification::warning(&error)
            }

            LogbookError::InvalidExpiryDate(date) => {
                let error = tid!("error.invalid-expiry-date", date: date.to_string());
                Notification::warning(&error)
            }

            LogbookError::NoMedications => {
                let error = tid!("error.no-medications");
                Notification::warning(&error)
            }

            LogbookError::DuplicateScript(id) => {
                let error = tid!("error.duplicate-script", id: id.to_string());
                Notification::error(&error)
            }

            LogbookError::InvalidScript(id) => {
                let error = tid!("error.invalid-script", id: id.to_string());
                Notification::error(&error)
            }

            LogbookError::InvalidDraftScript => {
                let error = tid!("error.invalid-draft-script");
                Notification::error(&error)
            }

            LogbookError::ScriptUsedInSupply => {
                let error = tid!("error.script-used-in-supply");
                Notification::warning(&error)
            }

            LogbookError::UnknownMedication(id) => {
                let error = tid!("error.unknown-medication", id: id.to_string());
                Notification::error(&error)
            }

            LogbookError::DuplicateSupply(id) => {
                let error = tid!("error.duplicate-supply", id: id.to_string());
                Notification::error(&error)
            }

            LogbookError::InvalidSupply(id) => {
                let error = tid!("error.invalid-supply", id: id.to_string());
                Notification::error(&error)
            }

            LogbookError::ScriptOutOfDate(id) => {
                let error = tid!("error.script-out-of-date", id: id.to_string());
                Notification::error(&error)
            }

            LogbookError::MedicationNotOnScript(script_id, medication_id) => {
                let error = tid!("error.medication-not-on-script", script_id: script_id.to_string(), medication_id: medication_id.to_string());
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

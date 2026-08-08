mod id;
mod level;
mod notification;

pub use id::NotificationId;
pub use level::NotificationLevel;
pub use notification::Notification;

// ------------------------------------
use dioxus::prelude::*;

#[component]
pub fn Notifications() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/css/notifications.css") },
        div {
            class: "notifications",
            NotificationsList {  }
        }
    }
}

#[component]
fn NotificationsList() -> Element {
    let notifications = use_context::<Signal<Vec<Notification>>>();

    rsx! {
        ul {
            class: "notifications__list",
            for notification in notifications() {
                NotificationsListItem { notification }
            }
        }
    }
}

#[component]
fn NotificationsListItem(notification: Notification) -> Element {
    let mut notifications = use_context::<Signal<Vec<Notification>>>();

    let class = notification.class();
    let id = notification.id();

    rsx! {
        li {
            class: "notifications__list-item {class}",
            key: "{id}",
            onclick: move |_| notifications.write().retain(|n| n.id() != id),
            "{notification}"
        }
    }
}

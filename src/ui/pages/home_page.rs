use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::application::Model;
use crate::application::View;
use crate::ui::components::{Medications, Prescriptions, Refills, Reminders};
#[component]
pub fn HomePage() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/css/home_page.css") }
        div {
            class: "home-page",
            Panel { view: View::Reminders, Reminders {} }
            Panel { view: View::Refills, Refills {} }
            Panel { view: View::Prescriptions, Prescriptions {} }
            Panel { view: View::Medications, Medications {} }
        }
    }
}

#[component]
fn Panel(view: View, children: Element) -> Element {
    let mut model = use_context::<Signal<Model>>();
    let current_view = model.read().view();

    let show_as_selected = current_view == View::HomePage || current_view == view;
    let show_children = current_view == view && current_view != View::HomePage;

    rsx! {
        div {
            class: "home-page__panel",
            class: "{view}",
            class: if show_as_selected { "selected" } else { "unselected" },
            class: if show_children { "selected-{view}" },

            h2 {
                onclick: move |_| model.write().set_view(view),
                {tid!(&format!("panel-name.{view}"))}
            }

            if show_children {
                {children}
            }
        }
    }
}

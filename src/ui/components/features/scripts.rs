use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::domain::{Logbook, Script, ScriptId};
use crate::ui::components::{AddButton, DeleteButton, EditButton};

#[component]
pub fn Scripts() -> Element {
    let selected_script_id = use_signal(|| None::<ScriptId>);
    provide_context(selected_script_id);

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/scripts.css")}
        div {
            class: "scripts",
            ScriptsList { }
            ScriptsCommands { }
        }

    }
}

#[component]
fn ScriptsList() -> Element {
    let logbook = use_context::<Signal<Logbook>>();

    let mut scripts = logbook.read().scripts().iter().cloned().collect::<Vec<_>>();
    scripts.sort_by_key(|s| s.issued_on());

    let zero_scripts = scripts.is_empty();

    rsx! {
        div {
           class: "scripts__list",
           for script in scripts {
               ScriptsListItem { script }
           }
       }
       if zero_scripts {
           p { {tid!("zero-scripts-para-01")} }
           p { {tid!("zero-scripts-para-02")} }
       }
    }
}

#[component]
fn ScriptsListItem(script: Script) -> Element {
    let script_id = script.id();
    let mut selected_script_id = use_context::<Signal<Option<ScriptId>>>();

    rsx! {
        div {
            class: "scripts__list-item",
            key: "{script_id}",
            onclick: move |_| selected_script_id.set(Some(script_id)),
            span { "{script.issued_on()} / {script.expires_on()}" }
        }

    }
}

#[component]
fn ScriptsCommands() -> Element {
    rsx! {
        div {
            class: "scripts__commands",
            AddButton {
                indefinite_object: tid!("script-indefinite"),
                onclick: move |_| { },
            }
            EditButton {
                definite_object: tid!("script-definite"),
                onclick: move |_| { },
            }
            DeleteButton {
                definite_object: tid!("script-definite"),
                onclick: move |_| { },
            }
        }
    }
}

use dioxus::prelude::*;

use crate::domain::Date;

#[component]
pub fn DateInput(
    id: String,
    label: String,
    value: Date,
    min: Option<Date>,
    max: Option<Date>,
    on_change: EventHandler<Date>,
) -> Element {
    let mut draft = use_signal(|| value.to_iso8601_string());
    let mut is_valid = use_signal(|| true);

    use_effect(move || {
        let result = Date::parse_iso8601_str(&draft.read());
        is_valid.set(result.is_ok());

        if let Ok(date) = result
            && date != value
        {
            on_change.call(date);
        }
    });

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/date_input.css") }
        label {
            class: "date-input",
            class: if !&*is_valid.read() { "date-input__error" },
            r#for: &id,
            span {
                class: "date-input__label",
                "{label}"
            }
            input {
                id: &id,
                r#type: "date",
                value: "{draft}",
                min: if let Some(min) = min { "{min}" },
                max: if let Some(max) = max { "{max}" },
                onchange: move |event| {
                    event.prevent_default();
                    draft.set(event.value());
                },
            }
        }
    }
}

use dioxus::prelude::*;
use dioxus_i18n::prelude::*;

use crate::application::Setup;
use crate::domain::{Date, LogbookSnapshot};
use crate::i18n;
use crate::storage::{self, PersistenceState};
use crate::ui::components::{HomePage, Notification, Notifications, TermsPage, WelcomePage};

#[component]
pub fn App() -> Element {
    let mut model = storage::use_application_model();
    provide_context(model);

    let notifications = use_signal(Vec::<Notification>::default);
    provide_context(notifications);

    let (mut logbook, state) = storage::use_logbook();
    provide_context(logbook);

    let mut housekeeping_required = use_signal(|| true);
    use_effect(move || {
        let is_idle = matches!(*state.read(), PersistenceState::Idle);
        if *housekeeping_required.peek() && is_idle {
            housekeeping_required.set(false);
            let today = Date::today();
            match logbook.write().housekeeping(today) {
                Ok(deleted) if deleted => Notification::message("housekeeping.complete"),
                Ok(_) => (),
                Err(error) => Notification::logbook_error(&error),
            }
        }
    });

    use_effect(move || {
        if let PersistenceState::Failed(error) = &*state.read() {
            Notification::storage_error(error);
        }
    });

    let logbook_snapshot = use_memo(move || LogbookSnapshot::from(&logbook.read(), Date::today()));
    provide_context(ReadSignal::from(logbook_snapshot));

    let language = i18n::use_preferred_language();
    use_effect(move || {
        if model.read().language().is_none() {
            model.write().set_language(language());
        }
    });

    let mut i18n = use_init_i18n(|| i18n::config(language.read().identifier()));
    use_effect(move || {
        if let Some(language) = model.read().language() {
            i18n.set_language(language.identifier());
        }
    });

    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        document::Stylesheet { href: asset!("/assets/css/main.css") }

        match model.read().setup() {
            Setup::Welcome => rsx! { WelcomePage {} },
            Setup::Terms => rsx! { TermsPage {} },
            Setup::Complete => rsx! { HomePage {} },
        }

        Notifications { }
    }
}

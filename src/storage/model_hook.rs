use dioxus::prelude::*;
use dioxus_sdk::storage::{use_storage, LocalStorage};

use crate::application::Model;

pub const STORAGE_KEY: &str = "script-pilot";

pub fn use_application_model() -> Signal<Model> {
    use_storage::<LocalStorage, _>(STORAGE_KEY.into(), Model::default)
}

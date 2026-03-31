use dioxus::prelude::*;
use crate::components::hero::Hero;
// =========================================
/// Home page
#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}

    }
}
// =========================================

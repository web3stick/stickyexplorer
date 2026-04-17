use dioxus::prelude::*;
use stickyexplorer::pages::route::Route;
// =========================================
const FAVICON: Asset = asset!("/assets/web_icon.svg");
const MAIN_CSS: Asset = asset!("/css/main.css");
const NAVBAR_CSS: Asset = asset!("/css/components_navbar.css");
const SEARCH_CSS: Asset = asset!("/css/components_search.css");
const HOME_CSS: Asset = asset!("/css/page_home.css");
const DETAIL_CSS: Asset = asset!("/css/page_detail.css");
const FOOTER_CSS: Asset = asset!("/css/footer.css");
const BUTTON_CSS: Asset = asset!("/css/button.css");
// =========================================
fn main() {
    dioxus::launch(App);
}

// =========================================
#[component]
fn App() -> Element {
    // Initialize dark mode signal
    let mut dark_mode_storage = use_signal(|| false);

    // Read dark mode preference from localStorage on mount
    use_effect(move || {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(stored)) = window.local_storage() {
                if let Ok(Some(value)) = stored.get_item("dark_mode") {
                    let is_dark = value == "true";
                    dark_mode_storage.set(is_dark);
                    // Apply the class immediately
                    if let Some(document) = window.document() {
                        if let Some(body) = document.body() {
                            if is_dark {
                                let _ = body.class_list().add_1("dark");
                            } else {
                                let _ = body.class_list().remove_1("dark");
                            }
                        }
                    }
                }
            }
        }
    });

    use_context_provider(|| dark_mode_storage);

    // hello log
    use_effect(|| {
        web_sys::console::log_1(&"👋 hello this app was made with ❤️ by sleet.near".into());
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }
        document::Link { rel: "stylesheet", href: SEARCH_CSS }
        document::Link { rel: "stylesheet", href: HOME_CSS }
        document::Link { rel: "stylesheet", href: DETAIL_CSS }
        document::Link { rel: "stylesheet", href: FOOTER_CSS }
        document::Link { rel: "stylesheet", href: BUTTON_CSS }
        Router::<Route> {}
    }
}
// =========================================

use dioxus::prelude::*;
use stickyexplorer::pages::route::Route;
// =========================================
const FAVICON: Asset = asset!("/assets/web_icon.svg");
const MAIN_CSS: Asset = asset!("/assets/css/main.css");
const NAVBAR_CSS: Asset = asset!("/assets/css/components_navbar.css");
const SEARCH_CSS: Asset = asset!("/assets/css/components_search.css");
const HOME_CSS: Asset = asset!("/assets/css/page_home.css");
const DETAIL_CSS: Asset = asset!("/assets/css/page_detail.css");
// =========================================
fn main() {
    dioxus::launch(App);
}
// =========================================
#[component]
fn App() -> Element {
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
        Router::<Route> {}
    }
}
// =========================================

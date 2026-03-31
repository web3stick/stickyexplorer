use dioxus::prelude::*;
use stickyexplorer::pages::route::Route;
// =========================================
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/css/main.css");
// =========================================
fn main() {
    dioxus::launch(App);
}
// =========================================
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}
// =========================================




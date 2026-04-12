// pages/route.rs
// =========================================
// Route definitions for NEAR Explorer
// =========================================
use crate::components::button_network::button_network;
use crate::components::search_bar::search_bar;
use crate::logic::network::NetworkId;
use crate::pages::page_account_detail::AccountDetail;
use crate::pages::page_block_detail::BlockDetail;
use crate::pages::page_home::Home as HomePage;
use crate::pages::page_tx_detail::TxDetail;
use dioxus::prelude::*;
// =========================================

#[component]
fn MainnetHome() -> Element {
    rsx! {
        HomePage {}
    }
}

#[component]
fn MainnetHomeRedirect() -> Element {
    let navigator = use_navigator();
    use_effect(move || {
        navigator.push("/mainnet");
    });
    rsx! {
        div { "Redirecting..." }
    }
}

#[component]
fn MainnetAccount(account_id: String) -> Element {
    rsx! {
        AccountDetail { account_id, network: NetworkId::Mainnet }
    }
}

#[component]
fn MainnetTx(tx_hash: String) -> Element {
    rsx! {
        TxDetail { tx_hash, network: NetworkId::Mainnet }
    }
}

#[component]
fn MainnetBlock(block_id: String) -> Element {
    rsx! {
        BlockDetail { block_id, network: NetworkId::Mainnet }
    }
}

#[component]
fn TestnetHomeRedirect() -> Element {
    let navigator = use_navigator();
    use_effect(move || {
        navigator.push("/testnet");
    });
    rsx! {
        div { "Redirecting..." }
    }
}

#[component]
fn TestnetAccount(account_id: String) -> Element {
    rsx! {
        AccountDetail { account_id, network: NetworkId::Testnet }
    }
}

#[component]
fn TestnetTx(tx_hash: String) -> Element {
    rsx! {
        TxDetail { tx_hash, network: NetworkId::Testnet }
    }
}

#[component]
fn TestnetBlock(block_id: String) -> Element {
    rsx! {
        BlockDetail { block_id, network: NetworkId::Testnet }
    }
}

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[layout(Navbar)]
    // Mainnet routes
    #[route("/")]
    MainnetHome {},
    #[route("/mainnet")]
    MainnetHomeRedirect {},
    #[route("/mainnet/account/:account_id")]
    MainnetAccount { account_id: String },
    #[route("/mainnet/tx/:tx_hash")]
    MainnetTx { tx_hash: String },
    #[route("/mainnet/block/:block_id")]
    MainnetBlock { block_id: String },
    // Testnet routes
    #[route("/testnet")]
    TestnetHomeRedirect {},
    #[route("/testnet/account/:account_id")]
    TestnetAccount { account_id: String },
    #[route("/testnet/tx/:tx_hash")]
    TestnetTx { tx_hash: String },
    #[route("/testnet/block/:block_id")]
    TestnetBlock { block_id: String },
    // Catch-all for unknown routes
    #[route("/:catchall")]
    CatchAll { catchall: String },
}

#[component]
pub fn Navbar() -> Element {
    let mut dark_mode = use_context::<Signal<bool>>();

    let toggle_dark_mode = move |_| {
        let new_value = !dark_mode();
        dark_mode.set(new_value);
        // Save to localStorage
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("dark_mode", if new_value { "true" } else { "false" });
            }
        }
        // Apply/remove dark class on body
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(body) = document.body() {
                    if new_value {
                        let _ = body.class_list().add_1("dark");
                    } else {
                        let _ = body.class_list().remove_1("dark");
                    }
                }
            }
        }
    };

    rsx! {
        header { id: "header",
            div {
                div { class: "navbar-left",
                    Link { to: Route::MainnetHome {}, class: "logo", "STICKYEXPLORER" }
                    button_network {}
                }
                div { class: "navbar-right",
                    button {
                        class: "dark-mode-toggle",
                        onclick: toggle_dark_mode,
                        if dark_mode() { "☀️" } else { "🌙" }
                    }
                    search_bar {}
                }
            }
        }
        main { class: "max-w-7xl mx-auto py-6", Outlet::<Route> {} }
    }
}
#[component]
fn CatchAll(catchall: String) -> Element {
    let navigator = use_navigator();
    use_effect(move || {
        navigator.push("/");
    });
    rsx! {
        div { class: "text-gray-500 text-center py-8", "Page not found — redirecting..." }
    }
}
// =========================================
// copyright 2026 by sleet.near

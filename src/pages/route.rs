// pages/route.rs
// =========================================
// Route definitions for NEAR Explorer
// =========================================
use crate::components::button_network::button_network;
use crate::components::search_bar::search_bar;
use crate::logic::network::{save_network_id, NetworkId};
use crate::pages::page_account_detail::AccountDetail;
use crate::pages::page_block_detail::BlockDetail;
use crate::pages::page_home::Home as HomePage;
use crate::pages::page_tx_detail::TxDetail;
use dioxus::prelude::*;
// =========================================

#[component]
fn MainnetHome() -> Element {
    use_effect(move || {
        let _ = save_network_id(NetworkId::Mainnet);
    });
    rsx! {
        HomePage {}
    }
}

#[component]
fn MainnetHomeRedirect() -> Element {
    use_effect(move || {
        let _ = save_network_id(NetworkId::Mainnet);
    });
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
    use_effect(move || {
        let _ = save_network_id(NetworkId::Mainnet);
    });
    rsx! {
        AccountDetail { account_id, network: NetworkId::Mainnet }
    }
}

#[component]
fn MainnetTx(tx_hash: String) -> Element {
    use_effect(move || {
        let _ = save_network_id(NetworkId::Mainnet);
    });
    rsx! {
        TxDetail { tx_hash, network: NetworkId::Mainnet }
    }
}

#[component]
fn MainnetBlock(block_id: String) -> Element {
    use_effect(move || {
        let _ = save_network_id(NetworkId::Mainnet);
    });
    rsx! {
        BlockDetail { block_id, network: NetworkId::Mainnet }
    }
}

#[component]
fn TestnetHomeRedirect() -> Element {
    use_effect(move || {
        let _ = save_network_id(NetworkId::Testnet);
    });
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
    use_effect(move || {
        let _ = save_network_id(NetworkId::Testnet);
    });
    rsx! {
        AccountDetail { account_id, network: NetworkId::Testnet }
    }
}

#[component]
fn TestnetTx(tx_hash: String) -> Element {
    use_effect(move || {
        let _ = save_network_id(NetworkId::Testnet);
    });
    rsx! {
        TxDetail { tx_hash, network: NetworkId::Testnet }
    }
}

#[component]
fn TestnetBlock(block_id: String) -> Element {
    use_effect(move || {
        let _ = save_network_id(NetworkId::Testnet);
    });
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
    rsx! {
        header { id: "header",
            div {
                div { class: "navbar-left",
                    Link { to: Route::MainnetHome {}, class: "logo", "STICKYEXPLORER" }
                    button_network {}
                }
                div { class: "navbar-right", search_bar {} }
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

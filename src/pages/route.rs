// pages/route.rs
// =========================================
// Route definitions for NEAR Explorer
// =========================================
use dioxus::prelude::*;
use crate::pages::page_home::Home as HomePage;
use crate::pages::page_account_detail::AccountDetail;
use crate::pages::page_tx_detail::TxDetail;
use crate::pages::page_block_detail::BlockDetail;
use crate::components::search_bar::search_bar;
use crate::components::button_network::button_network;
use crate::logic::network::{NetworkId, save_network_id};
// =========================================

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[route("/account/:account_id")]
        Account { account_id: String },
        #[route("/tx/:tx_hash")]
        Tx { tx_hash: String },
        #[route("/block/:block_id")]
        Block { block_id: String },
        // Mainnet routes
        #[route("/mainnet")]
        MainnetHome {},
        #[route("/mainnet/account/:account_id")]
        MainnetAccount { account_id: String },
        #[route("/mainnet/tx/:tx_hash")]
        MainnetTx { tx_hash: String },
        #[route("/mainnet/block/:block_id")]
        MainnetBlock { block_id: String },
        // Testnet routes
        #[route("/testnet")]
        TestnetHome {},
        #[route("/testnet/account/:account_id")]
        TestnetAccount { account_id: String },
        #[route("/testnet/tx/:tx_hash")]
        TestnetTx { tx_hash: String },
        #[route("/testnet/block/:block_id")]
        TestnetBlock { block_id: String },
}

#[component]
pub fn Navbar() -> Element {
    rsx! {
        header {
            id: "header",
            div {
                div { class: "navbar-left",
                    Link {
                        to: Route::Home {},
                        class: "logo",
                        "StickyExplorer"
                    }
                    button_network {}
                }
                div { class: "navbar-right",
                    search_bar {}
                }
            }
        }
        main {
            class: "max-w-7xl mx-auto py-6",
            Outlet::<Route> {}
        }
    }
}

#[component]
fn Home() -> Element {
    rsx! { HomePage {} }
}

#[component]
fn Account(account_id: String) -> Element {
    rsx! { AccountDetail { account_id } }
}

#[component]
fn Tx(tx_hash: String) -> Element {
    rsx! { TxDetail { tx_hash } }
}

#[component]
fn Block(block_id: String) -> Element {
    rsx! { BlockDetail { block_id } }
}

#[component]
fn MainnetHome() -> Element {
    use_effect(move || {
        let _ = save_network_id(NetworkId::Mainnet);
    });
    rsx! { HomePage {} }
}

#[component]
fn MainnetAccount(account_id: String) -> Element {
    rsx! { AccountDetail { account_id } }
}

#[component]
fn MainnetTx(tx_hash: String) -> Element {
    rsx! { TxDetail { tx_hash } }
}

#[component]
fn MainnetBlock(block_id: String) -> Element {
    rsx! { BlockDetail { block_id } }
}

#[component]
fn TestnetHome() -> Element {
    use_effect(move || {
        let _ = save_network_id(NetworkId::Testnet);
    });
    rsx! { HomePage {} }
}

#[component]
fn TestnetAccount(account_id: String) -> Element {
    rsx! { AccountDetail { account_id } }
}

#[component]
fn TestnetTx(tx_hash: String) -> Element {
    rsx! { TxDetail { tx_hash } }
}

#[component]
fn TestnetBlock(block_id: String) -> Element {
    rsx! { BlockDetail { block_id } }
}
// =========================================
// copyright 2026 by sleet.near

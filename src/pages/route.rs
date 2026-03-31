// pages/route.rs
// =========================================
// Route definitions for NEAR Explorer
// =========================================
use dioxus::prelude::*;
use crate::pages::page_account_detail::AccountDetail;
use crate::pages::page_tx_detail::TxDetail;
use crate::pages::page_block_detail::BlockDetail;
use crate::components::search_bar::search_bar;
use crate::components::button_network::button_network;
// =========================================

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[route("/account/:account_id")]
        AccountView { account_id: String },
        #[route("/tx/:tx_hash")]
        TxView { tx_hash: String },
        #[route("/block/:block_id")]
        BlockView { block_id: String },
}

#[component]
pub fn Navbar() -> Element {
    rsx! {
        header {
            id: "header",
            class: "max-w-7xl mx-auto mb-6",
            div {
                class: "flex flex-col sm:flex-row items-center gap-4 py-4 border-b border-gray-200 pb-4",
                Link {
                    to: Route::Home {},
                    class: "text-xl font-bold hover:text-[#8CA2F5] transition-colors",
                    "StickyExplorer"
                }
                div { class: "flex-1 w-full sm:max-w-md",
                    search_bar {}
                }
                div {
                    button_network {}
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
    rsx! {
        crate::pages::page_home::Home {}
    }
}

#[component]
fn AccountView(account_id: String) -> Element {
    rsx! {
        AccountDetail { account_id }
    }
}

#[component]
fn TxView(tx_hash: String) -> Element {
    rsx! {
        TxDetail { tx_hash }
    }
}

#[component]
fn BlockView(block_id: String) -> Element {
    rsx! {
        BlockDetail { block_id }
    }
}
// =========================================
// copyright 2026 by sleet.near

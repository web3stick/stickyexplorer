// components/search_bar.rs
// =========================================
// Search bar component with network auto-switching
// =========================================
use dioxus::prelude::*;
use crate::logic::network::{NetworkId, save_network_id, get_stored_network_id};
use crate::utils::format::decode_base58;
// =========================================

/// Detect the type of search query
fn detect_type(q: &str) -> Option<&'static str> {
    if q.is_empty() {
        return None;
    }
    let stripped = q.replace(',', "");
    if stripped.chars().all(|c| c.is_ascii_digit()) {
        return Some("block");
    }
    if q.len() < 50 {
        if let Some(bytes) = decode_base58(q) {
            if bytes.len() == 32 {
                return Some("tx");
            }
        }
    }
    Some("account")
}

#[component]
pub fn search_bar() -> Element {
    let mut query = use_signal(|| String::new());
    let network_id = use_signal(|| get_stored_network_id());
    let navigator = use_navigator();

    let search_type = use_memo(move || detect_type(&query()));

    let handle_search = move |e: Event<FormData>| {
        e.prevent_default();
        let q = query().trim().to_string();
        let search_type = search_type();

        if q.is_empty() || search_type.is_none() {
            return;
        }

        match search_type.unwrap() {
            "block" => {
                let stripped = q.replace(',', "");
                navigator.push(format!("/block/{}", stripped));
            }
            "tx" => {
                navigator.push(format!("/tx/{}", q));
            }
            "account" => {
                // Check for network auto-switching
                let detected_network = NetworkId::from_account_id(&q);
                let current_network = network_id();
                
                if detected_network != current_network {
                    // Save the new network and redirect to the other network
                    let _ = save_network_id(detected_network);
                    // For cross-network redirects, we need to change the domain
                    // This is a simplified version - in production you'd use actual domains
                    let other_network_url = current_network.other_network_url();
                    let redirect_url = format!("{}/account/{}", other_network_url, q);
                    // Use window.location for cross-domain redirect
                    if let Some(window) = web_sys::window() {
                        let _ = window.location().set_href(&redirect_url);
                    }
                } else {
                    navigator.push(format!("/account/{}", q));
                }
            }
            _ => {}
        }

        query.set(String::new());
    };

    let hint_label = match search_type() {
        Some("block") => "Block",
        Some("tx") => "Transaction",
        Some("account") => "Account",
        _ => "",
    };

    rsx! {
        form {
            onsubmit: handle_search,
            class: "flex gap-2",
            div {
                class: "relative flex-1",
                input {
                    r#type: "text",
                    value: "{query}",
                    oninput: move |e| query.set(e.value()),
                    placeholder: "Search tx, block, or account",
                    class: "w-full rounded-lg border border-gray-300 bg-white px-4 py-2 pr-20 text-sm focus:border-blue-500 focus:outline-none",
                }
                if let Some(_hint) = search_type() {
                    span {
                        class: "absolute right-3 top-1/2 -translate-y-1/2 text-xs text-gray-400",
                        "{hint_label}"
                    }
                }
            }
            button {
                r#type: "submit",
                class: "rounded-lg bg-[#8CA2F5] px-3 py-2 sm:px-4 text-sm font-medium text-white hover:bg-[#7a91e8]",
                svg {
                    class: "size-4 sm:hidden",
                    fill: "none",
                    stroke: "currentColor",
                    view_box: "0 0 24 24",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z",
                    }
                }
                span { class: "hidden sm:inline", "Search" }
            }
        }
    }
}
// =========================================
// copyright 2026 by sleet.near

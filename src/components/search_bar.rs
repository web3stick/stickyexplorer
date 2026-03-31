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
            class: "search-form",
            div {
                class: "search-input-wrapper",
                input {
                    r#type: "text",
                    value: "{query}",
                    oninput: move |e| query.set(e.value()),
                    placeholder: "Search tx, block, or account",
                }
                if let Some(_hint) = search_type() {
                    span {
                        class: "search-hint",
                        "{hint_label}"
                    }
                }
            }
            button {
                r#type: "submit",
                "GO"
            }
        }
    }
}
// =========================================
// copyright 2026 by sleet.near

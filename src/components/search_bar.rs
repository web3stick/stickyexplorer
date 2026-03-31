// components/search_bar.rs
// =========================================
// Search bar component with network auto-switching
// =========================================
use dioxus::prelude::*;
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
            "account" => {
                // Auto-switch network based on account suffix
                // .testnet → testnet, everything else (.near, .tg, etc.) → mainnet
                if q.ends_with(".testnet") {
                    navigator.push(format!("/testnet/account/{}", q));
                } else {
                    navigator.push(format!("/mainnet/account/{}", q));
                }
            }
            "block" => {
                // Blocks are network-specific, default to mainnet
                let stripped = q.replace(',', "");
                navigator.push(format!("/mainnet/block/{}", stripped));
            }
            "tx" => {
                // Transactions are network-specific, default to mainnet  
                navigator.push(format!("/mainnet/tx/{}", q));
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

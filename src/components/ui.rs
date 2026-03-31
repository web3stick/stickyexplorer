// components/ui.rs
// =========================================
// Shared UI components for NEAR Explorer
// =========================================
use dioxus::prelude::*;
use crate::utils::format::{format_gas_amount, format_near_amount, format_time_ago, truncate_middle};
// =========================================

/// Account ID display component
#[component]
pub fn account_id(account_id: String, #[props(default)] max_length: Option<usize>) -> Element {
    let display = max_length
        .map(|len| truncate_middle(&account_id, len))
        .unwrap_or_else(|| account_id.clone());

    rsx! {
        Link {
            to: format!("/account/{}", account_id),
            class: "font-mono text-sm hover:underline",
            "{display}"
        }
    }
}

/// Transaction hash display component
#[component]
pub fn transaction_hash(hash: String, #[props(default)] truncate: Option<bool>) -> Element {
    let display = if truncate.unwrap_or(true) {
        truncate_middle(&hash, 12)
    } else {
        hash.clone()
    };

    rsx! {
        Link {
            to: format!("/tx/{}", hash),
            class: "font-mono text-sm hover:underline",
            "{display}"
        }
    }
}

/// Block height display component
#[component]
pub fn block_height(height: u64) -> Element {
    rsx! {
        Link {
            to: format!("/block/{}", height),
            class: "font-mono text-sm hover:underline",
            "{height}"
        }
    }
}

/// Block hash display component
#[component]
pub fn block_hash(hash: String) -> Element {
    rsx! {
        span {
            class: "font-mono text-sm",
            "{truncate_middle(&hash, 12)}"
        }
    }
}

/// Time ago component
#[component]
pub fn time_ago(timestamp_ns: String) -> Element {
    rsx! {
        span {
            class: "text-gray-500 text-sm",
            "{format_time_ago(&timestamp_ns)}"
        }
    }
}

/// Gas amount display component
#[component]
pub fn gas_amount(gas: u64) -> Element {
    rsx! {
        span {
            class: "font-mono text-xs",
            "{format_gas_amount(gas)}"
        }
    }
}

/// NEAR amount display component
#[component]
pub fn near_amount(yocto_near: String, #[props(default)] show_price: Option<bool>) -> Element {
    let formatted = format_near_amount(&yocto_near);
    let price_display = if show_price.unwrap_or(false) {
        let price = formatted.parse::<f64>().unwrap_or(0.0) * 5.0;
        rsx! { span { class: "text-gray-500", " (~${price:.2})" } }
    } else {
        rsx! {}
    };
    
    rsx! {
        span {
            class: "font-mono text-sm",
            "{formatted} NEAR"
            {price_display}
        }
    }
}
// =========================================
// copyright 2026 by sleet.near

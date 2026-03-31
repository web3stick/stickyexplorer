// components/widgets.rs
// =========================================
// Transaction widget system
// =========================================
use dioxus::prelude::*;
use crate::api::types::TransactionDetail;
use crate::utils::parse_transaction::parse_transaction;
use crate::components::ui::{account_id, near_amount};
// =========================================

/// Widget match function type
pub type MatchFn = fn(&TransactionDetail) -> bool;

/// Widget component type
pub type WidgetFn = fn(Element, &TransactionDetail) -> Element;

/// Widget definition
pub struct Widget {
    pub id: &'static str,
    pub match_fn: MatchFn,
    pub render: fn(&TransactionDetail) -> Element,
    pub widget_type: WidgetType,
}

#[derive(Clone, Copy, PartialEq)]
pub enum WidgetType {
    Explanation,  // Renders above receipts
    Utility,      // Renders below receipts
}

/// Match FT transfer transactions
pub fn match_ft_transfer(tx: &TransactionDetail) -> bool {
    let parsed = parse_transaction(tx);
    // Check if there are any NEP-141 transfers
    !parsed.transfers.is_empty() && parsed.transfers.iter().any(|t| {
        matches!(t.token_type, crate::utils::parse_transaction::TokenType::Nep141)
    })
}

/// Render FT transfer widget
pub fn render_ft_transfer(tx: &TransactionDetail) -> Element {
    let parsed = parse_transaction(tx);
    let ft_transfers: Vec<_> = parsed.transfers.iter()
        .filter(|t| matches!(t.token_type, crate::utils::parse_transaction::TokenType::Nep141))
        .collect();
    
    let status_class = match parsed.is_success {
        None => "border-yellow-200 bg-yellow-50",
        Some(true) => "border-green-200 bg-green-50",
        Some(false) => "border-red-200 bg-red-50",
    };

    rsx! {
        div {
            class: "mb-4 rounded-lg border {status_class} p-4",
            div { class: "flex flex-col gap-1",
                span { class: "font-medium text-sm", "Token Transfer" }
                for transfer in ft_transfers.iter() {
                    span { class: "flex flex-wrap items-center gap-1 text-sm",
                        if let Some(from) = &transfer.from {
                            account_id { account_id: from.clone() }
                            span { class: "text-gray-500", "→" }
                        }
                        if let Some(to) = &transfer.to {
                            account_id { account_id: to.clone() }
                        }
                        span { class: "font-mono text-xs", "{transfer.amount}" }
                        if let Some(contract) = &transfer.contract_id {
                            span { class: "text-gray-500 text-xs", "({contract})" }
                        }
                    }
                }
            }
        }
    }
}

/// Match NEAR transfer transactions
pub fn match_near_transfer(tx: &TransactionDetail) -> bool {
    let parsed = parse_transaction(tx);
    parsed.actions.len() == 1 
        && parsed.actions.first().map_or(false, |a| a.action_type == "Transfer")
        && parsed.actions.first().and_then(|a| a.deposit.as_ref()).map_or(false, |d| d != "0")
}

/// Render NEAR transfer widget
pub fn render_near_transfer(tx: &TransactionDetail) -> Element {
    let parsed = parse_transaction(tx);
    let deposit = parsed.actions.first().and_then(|a| a.deposit.clone()).unwrap_or_default();
    
    let status_class = match parsed.is_success {
        None => "border-yellow-200 bg-yellow-50",
        Some(true) => "border-green-200 bg-green-50",
        Some(false) => "border-red-200 bg-red-50",
    };

    rsx! {
        div {
            class: "mb-4 rounded-lg border {status_class} p-4",
            div { class: "flex flex-col gap-1",
                span { class: "flex flex-wrap items-center gap-1",
                    account_id { account_id: parsed.signer_id.clone() }
                    span { class: "text-gray-500", "transferred" }
                    span { class: "font-semibold",
                        near_amount { yocto_near: deposit }
                    }
                    span { class: "text-gray-500", "to" }
                    account_id { account_id: parsed.receiver_id.clone() }
                }
            }
        }
    }
}

/// Default widget - shows raw JSON
pub fn render_default_widget(tx: &TransactionDetail) -> Element {
    let mut open = use_signal(|| false);  // Closed by default
    let tx_json = serde_json::to_string_pretty(tx).unwrap_or_default();

    rsx! {
        div {
            class: "mt-6 rounded-lg border border-gray-200 bg-white",
            button {
                onclick: move |_| open.toggle(),
                class: "flex w-full cursor-pointer items-center gap-2 px-4 py-2.5 text-sm font-medium text-gray-600 hover:bg-gray-50 hover:text-gray-900 transition-colors",
                span { "Raw JSON" }
            }
            if open() {
                div {
                    class: "border-t border-gray-100",
                    div {
                        class: "p-4",
                        div {
                            class: "json-container",
                            pre {
                                "{tx_json}"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Get matching widgets for a transaction
pub fn get_matching_widgets(tx: &TransactionDetail) -> Vec<&'static Widget> {
    static WIDGETS: &[Widget] = &[
        Widget {
            id: "near-transfer",
            match_fn: match_near_transfer,
            render: render_near_transfer,
            widget_type: WidgetType::Explanation,
        },
        Widget {
            id: "ft-transfer",
            match_fn: match_ft_transfer,
            render: render_ft_transfer,
            widget_type: WidgetType::Explanation,
        },
        Widget {
            id: "default",
            match_fn: |_| true,
            render: render_default_widget,
            widget_type: WidgetType::Utility,
        },
    ];
    
    WIDGETS.iter().filter(|w| (w.match_fn)(tx)).collect()
}
// =========================================
// copyright 2026 by sleet.near

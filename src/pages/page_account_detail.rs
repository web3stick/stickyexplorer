// pages/page_account_detail.rs
// =========================================
// Account detail page with transactions
// =========================================
use dioxus::prelude::*;
use reqwest::Client;
use serde::Serialize;
use crate::api::types::{AccountFilters, AccountTx};
use crate::components::ui::{time_ago, transaction_hash};
use crate::logic::network::get_stored_network_id;
// =========================================

const BATCH_SIZE: u32 = 80;

#[derive(Clone, Serialize)]
struct AccountParams<'a> {
    account_id: &'a str,
    #[serde(flatten)]
    filters: &'a AccountFilters,
    #[serde(skip_serializing_if = "Option::is_none")]
    resume_token: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

#[component]
pub fn AccountDetail(account_id: String) -> Element {
    let mut txs = use_signal(|| Vec::<AccountTx>::new());
    let mut loading = use_signal(|| true);
    let mut error = use_signal(|| Option::<String>::None);
    let mut resume_token = use_signal(|| Option::<String>::None);
    let mut has_more = use_signal(|| true);
    let mut loading_more = use_signal(|| false);
    let mut txs_count = use_signal(|| 0u64);

    let network_id = get_stored_network_id();
    let api_base = network_id.api_base_url();
    
    // Clone account_id for closures before moving
    let account_id_for_effect = account_id.clone();
    let account_id_for_load_more = account_id.clone();
    let account_id_display = account_id.clone();

    // Initial load
    use_effect(move || {
        let api_base = api_base.to_string();
        let account_id = account_id_for_effect.clone();
        spawn(async move {
            loading.set(true);
            error.set(None);
            
            let client = Client::new();
            let filters = AccountFilters::default();
            let params = AccountParams {
                account_id: &account_id,
                filters: &filters,
                resume_token: None,
                limit: Some(BATCH_SIZE),
            };

            match client
                .post(format!("{}/v0/account", api_base))
                .json(&params)
                .send()
                .await
            {
                Ok(resp) => {
                    if let Ok(data) = resp.json::<serde_json::Value>().await {
                        if let Some(account_txs) = data.get("account_txs").and_then(|v| v.as_array()) {
                            let new_txs: Vec<AccountTx> = account_txs
                                .iter()
                                .filter_map(|v| serde_json::from_value(v.clone()).ok())
                                .collect();
                            
                            txs_count.set(data.get("txs_count").and_then(|v| v.as_u64()).unwrap_or(0));
                            
                            if !new_txs.is_empty() {
                                resume_token.set(data.get("resume_token").and_then(|v| v.as_str()).map(String::from));
                                if new_txs.len() < BATCH_SIZE as usize {
                                    has_more.set(false);
                                }
                            } else {
                                has_more.set(false);
                            }
                            txs.set(new_txs);
                        }
                    }
                }
                Err(e) => {
                    error.set(Some(e.to_string()));
                }
            }
            loading.set(false);
        });
    });

    let load_more = move |_| {
        if !has_more() || loading_more() {
            return;
        }
        
        let api_base = api_base.to_string();
        let account_id = account_id_for_load_more.clone();
        let token = resume_token();
        loading_more.set(true);

        spawn(async move {
            let client = Client::new();
            let filters = AccountFilters::default();
            let params = AccountParams {
                account_id: &account_id,
                filters: &filters,
                resume_token: token.as_deref(),
                limit: Some(BATCH_SIZE),
            };

            if let Ok(resp) = client
                .post(format!("{}/v0/account", api_base))
                .json(&params)
                .send()
                .await
            {
                if let Ok(data) = resp.json::<serde_json::Value>().await {
                    if let Some(account_txs) = data.get("account_txs").and_then(|v| v.as_array()) {
                        let new_txs: Vec<AccountTx> = account_txs
                            .iter()
                            .filter_map(|v| serde_json::from_value(v.clone()).ok())
                            .collect();
                        
                        if new_txs.is_empty() || new_txs.len() < BATCH_SIZE as usize {
                            has_more.set(false);
                        }
                        
                        if let Some(resume) = data.get("resume_token").and_then(|v| v.as_str()) {
                            resume_token.set(Some(resume.to_string()));
                        }
                        
                        txs.write().extend(new_txs);
                    }
                }
            }
            loading_more.set(false);
        });
    };

    let txs_count_val = txs_count();
    let txs_list = txs();
    let has_more_val = has_more();
    let loading_more_val = loading_more();
    let loading_val = loading();
    let error_val = error();
    let txs_list_for_desktop = txs_list.clone();
    let txs_list_for_mobile = txs_list.clone();
    let txs_list_empty = txs_list.is_empty();

    if let Some(err) = error_val {
        return rsx! {
            p { class: "text-red-600", "Error loading account: {err}" }
        };
    }

    rsx! {
        div {
            h1 { class: "mb-4 text-xl font-bold",
                "Account: "
                span { class: "font-mono text-base", "{account_id_display}" }
            }

            if txs_count_val > 0 {
                p { class: "mb-3 text-sm text-gray-600",
                    "Transactions ({txs_count_val})"
                }
            }

            // Desktop table
            div { class: "table-container",
                table {
                    thead {
                        tr {
                            th { "Tx Hash" }
                            th { "Time" }
                            th { "Signer" }
                            th { "Receiver" }
                            th { "Action" }
                            th { "Status" }
                        }
                    }
                    tbody {
                        for tx in txs_list_for_desktop {
                            tr {
                                td {
                                    transaction_hash { hash: tx.transaction_hash.clone() }
                                }
                                td { class: "text-gray-500",
                                    time_ago { timestamp_ns: tx.tx_block_timestamp.clone() }
                                }
                                td {
                                    if tx.is_signer || tx.is_real_signer {
                                        span { class: "font-mono text-xs", "{account_id_display}" }
                                    } else {
                                        span { class: "text-gray-400 text-xs", "—" }
                                    }
                                }
                                td {
                                    if tx.is_receiver || tx.is_real_receiver {
                                        span { class: "font-mono text-xs", "{account_id_display}" }
                                    } else {
                                        span { class: "text-gray-400 text-xs", "—" }
                                    }
                                }
                                td {
                                    if tx.is_function_call {
                                        span { class: "text-xs", "Function Call" }
                                    } else if tx.is_delegated_signer {
                                        span { class: "text-xs", "Delegate" }
                                    } else {
                                        span { class: "text-xs text-gray-400", "Transfer" }
                                    }
                                }
                                td {
                                    if tx.is_success {
                                        span { class: "status-success", "✓" }
                                    } else {
                                        span { class: "status-failed", "✗" }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Mobile cards
            div { class: "mobile-cards",
                for tx in txs_list_for_mobile {
                    div {
                        div { class: "flex items-center justify-between gap-2 mb-1",
                            span { class: "font-mono text-xs",
                                transaction_hash { hash: tx.transaction_hash.clone() }
                            }
                            if tx.is_success {
                                span { class: "status-success text-xs", "✓" }
                            } else {
                                span { class: "status-failed text-xs", "✗" }
                            }
                        }
                        div { class: "text-sm text-gray-500 mb-1",
                            time_ago { timestamp_ns: tx.tx_block_timestamp.clone() }
                        }
                        div { class: "flex flex-col gap-1 text-sm",
                            div {
                                span { class: "text-gray-500 text-xs", "Signer: " }
                                if tx.is_signer || tx.is_real_signer {
                                    span { class: "font-mono text-xs", "{account_id_display}" }
                                } else {
                                    span { class: "text-gray-400 text-xs", "—" }
                                }
                            }
                            div {
                                span { class: "text-gray-500 text-xs", "Receiver: " }
                                if tx.is_receiver || tx.is_real_receiver {
                                    span { class: "font-mono text-xs", "{account_id_display}" }
                                } else {
                                    span { class: "text-gray-400 text-xs", "—" }
                                }
                            }
                            div {
                                span { class: "text-gray-500 text-xs", "Action: " }
                                if tx.is_function_call {
                                    span { class: "text-xs", "Function Call" }
                                } else if tx.is_delegated_signer {
                                    span { class: "text-xs", "Delegate" }
                                } else {
                                    span { class: "text-xs text-gray-400", "Transfer" }
                                }
                            }
                        }
                    }
                }
            }

            if !loading_val && txs_list_empty {
                p { class: "empty-state", "No transactions found" }
            }

            // Load more button - centered
            if has_more_val {
                div { class: "load-more-container",
                    button {
                        onclick: load_more,
                        disabled: loading_more_val,
                        class: "load-more-button",
                        if loading_more_val { "Loading..." } else { "Load More" }
                    }
                }
            }
        }
    }
}
// =========================================
// copyright 2026 by sleet.near

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
use crate::logic::tx_cache::TxCache;
use crate::utils::parse_transaction::{parse_transaction, ParsedTx};
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

#[derive(Clone, Serialize)]
struct TxParams {
    tx_hashes: Vec<String>,
}

#[component]
pub fn AccountDetail(account_id: String) -> Element {
    let network_id = get_stored_network_id();
    let api_base = network_id.api_base_url();
    let account_id_for_resource = account_id.clone();
    
    // Global cache for faster navigation
    let tx_cache = use_signal(|| TxCache::new());
    
    // Use resource for data fetching - automatically re-runs when account_id changes
    let account_data = use_resource(move || {
        let api_base = api_base.to_string();
        let account_id = account_id_for_resource.clone();
        let mut tx_cache = tx_cache.clone();
        
        async move {
            let client = Client::new();
            let filters = AccountFilters::default();
            let params = AccountParams {
                account_id: &account_id,
                filters: &filters,
                resume_token: None::<&str>,
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
                            
                            let txs_count = data.get("txs_count").and_then(|v| v.as_u64()).unwrap_or(0);
                            
                            // Fetch full transaction details (with caching)
                            let hashes: Vec<String> = new_txs.iter().map(|t| t.transaction_hash.clone()).collect();
                            let parsed = fetch_and_parse_transactions(&api_base, &hashes, &mut tx_cache).await;
                            
                            return Some((new_txs, parsed, txs_count));
                        }
                    }
                }
                Err(_) => {}
            }
            None
        }
    });

    let account_id_display = account_id.clone();
    
    // Read the resource state
    let data = account_data.read();
    
    if data.is_none() || data.as_ref().unwrap().is_none() {
        return rsx! {
            div { class: "empty-state", "Loading account..." }
        };
    }
    
    let inner_data = data.as_ref().unwrap().as_ref().unwrap();
    let txs = &inner_data.0;
    let parsed_list = &inner_data.1;
    let txs_count = inner_data.2;
    
    let parsed_map: std::collections::HashMap<String, ParsedTx> = parsed_list.iter().cloned().collect();
    let txs_list_empty = txs.is_empty();
    let txs_count_val = txs_count;

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
                        for atx in txs.iter() {
                            if let Some(parsed) = parsed_map.get(&atx.transaction_hash) {
                                tr {
                                    td {
                                        transaction_hash { hash: atx.transaction_hash.clone() }
                                    }
                                    td { class: "text-gray-500",
                                        time_ago { timestamp_ns: atx.tx_block_timestamp.clone() }
                                    }
                                    td {
                                        span { class: "font-mono text-xs", "{parsed.signer_id}" }
                                    }
                                    td {
                                        span { class: "font-mono text-xs", "{parsed.receiver_id}" }
                                    }
                                    td {
                                        if let Some(first_action) = parsed.actions.first() {
                                            span { class: "text-xs",
                                                "{first_action.action_type}"
                                                if let Some(ref method) = first_action.method_name {
                                                    "::"
                                                    "{method}"
                                                }
                                            }
                                        } else {
                                            span { class: "text-xs text-gray-400", "Unknown" }
                                        }
                                    }
                                    td {
                                        if let Some(success) = parsed.is_success {
                                            if success {
                                                span { class: "status-success", "✓" }
                                            } else {
                                                span { class: "status-failed", "✗" }
                                            }
                                        } else {
                                            span { class: "status-pending", "⏳" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Mobile cards
            div { class: "mobile-cards",
                for atx in txs.iter() {
                    if let Some(parsed) = parsed_map.get(&atx.transaction_hash) {
                        div {
                            div { class: "flex items-center justify-between gap-2 mb-1",
                                span { class: "font-mono text-xs",
                                    transaction_hash { hash: atx.transaction_hash.clone() }
                                }
                                if let Some(success) = parsed.is_success {
                                    if success {
                                        span { class: "status-success text-xs", "✓" }
                                    } else {
                                        span { class: "status-failed text-xs", "✗" }
                                    }
                                } else {
                                    span { class: "status-pending text-xs", "⏳" }
                                }
                            }
                            div { class: "text-sm text-gray-500 mb-1",
                                time_ago { timestamp_ns: atx.tx_block_timestamp.clone() }
                            }
                            div { class: "flex flex-col gap-1 text-sm",
                                div {
                                    span { class: "text-gray-500 text-xs", "Signer: " }
                                    span { class: "font-mono text-xs", "{parsed.signer_id}" }
                                }
                                div {
                                    span { class: "text-gray-500 text-xs", "Receiver: " }
                                    span { class: "font-mono text-xs", "{parsed.receiver_id}" }
                                }
                                div {
                                    span { class: "text-gray-500 text-xs", "Action: " }
                                    if let Some(first_action) = parsed.actions.first() {
                                        span { class: "text-xs",
                                            "{first_action.action_type}"
                                            if let Some(ref method) = first_action.method_name {
                                                "::"
                                                "{method}"
                                            }
                                        }
                                    } else {
                                        span { class: "text-xs text-gray-400", "Unknown" }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if txs_list_empty {
                p { class: "empty-state", "No transactions found" }
            }
        }
    }
}

async fn fetch_and_parse_transactions(api_base: &str, hashes: &[String], tx_cache: &mut Signal<TxCache>) -> Vec<(String, ParsedTx)> {
    if hashes.is_empty() {
        return Vec::new();
    }
    
    // First check cache for existing transactions
    let mut all_parsed = Vec::new();
    let mut missing_hashes = Vec::new();
    
    for hash in hashes {
        if let Some(parsed) = tx_cache.read().get(hash) {
            all_parsed.push((hash.clone(), parsed.clone()));
        } else {
            missing_hashes.push(hash.clone());
        }
    }
    
    // If we have all in cache, return early
    if missing_hashes.is_empty() {
        return all_parsed;
    }
    
    const BATCH_SIZE: usize = 20;
    
    // Process missing in batches to avoid API limits
    for chunk in missing_hashes.chunks(BATCH_SIZE) {
        let client = Client::new();
        let params = TxParams {
            tx_hashes: chunk.to_vec(),
        };
        
        match client
            .post(format!("{}/v0/transactions", api_base))
            .json(&params)
            .send()
            .await
        {
            Ok(resp) => {
                if let Ok(data) = resp.json::<serde_json::Value>().await {
                    if let Some(tx_array) = data.get("transactions").and_then(|v| v.as_array()) {
                        let parsed: Vec<(String, ParsedTx)> = tx_array
                            .iter()
                            .filter_map(|v| {
                                serde_json::from_value::<crate::api::types::TransactionDetail>(v.clone()).ok()
                            })
                            .map(|tx| {
                                let parsed = parse_transaction(&tx);
                                (parsed.hash.clone(), parsed.clone())
                            })
                            .collect();
                        
                        // Add to cache
                        tx_cache.write().insert_batch(parsed.clone());
                        all_parsed.extend(parsed);
                    }
                }
            }
            Err(_) => {}
        }
        
        // Small delay between batches to avoid rate limiting
        gloo_timers::future::TimeoutFuture::new(50).await;
    }
    
    all_parsed
}
// =========================================
// copyright 2026 by sleet.near

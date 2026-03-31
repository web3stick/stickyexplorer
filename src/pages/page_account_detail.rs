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
    let mut txs = use_signal(|| Vec::<AccountTx>::new());
    let mut parsed_txs = use_signal(|| Vec::<(String, ParsedTx)>::new());
    let mut loading = use_signal(|| true);
    let mut error = use_signal(|| Option::<String>::None);
    let mut resume_token = use_signal(|| Option::<String>::None);
    let mut has_more = use_signal(|| true);
    let mut loading_more = use_signal(|| false);
    let mut txs_count = use_signal(|| 0u64);
    
    // Global cache for faster navigation
    let tx_cache = use_signal(|| TxCache::new());

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
                            
                            // Fetch full transaction details (with caching)
                            let hashes: Vec<String> = new_txs.iter().map(|t| t.transaction_hash.clone()).collect();
                            let parsed = fetch_and_parse_transactions(&api_base, &hashes, tx_cache).await;
                            txs.set(new_txs);
                            parsed_txs.set(parsed);
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
                        
                        // Fetch full transaction details for new txs (with caching)
                        let hashes: Vec<String> = new_txs.iter().map(|t| t.transaction_hash.clone()).collect();
                        let parsed = fetch_and_parse_transactions(&api_base, &hashes, tx_cache).await;
                        
                        txs.write().extend(new_txs);
                        parsed_txs.write().extend(parsed);
                    }
                }
            }
            loading_more.set(false);
        });
    };

    let txs_count_val = txs_count();
    let txs_list = txs();
    let parsed_list = parsed_txs();
    let has_more_val = has_more();
    let loading_more_val = loading_more();
    let loading_val = loading();
    let error_val = error();
    
    // Create a map for quick lookup
    let parsed_map: std::collections::HashMap<String, ParsedTx> = parsed_list.into_iter().collect();
    
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
                        for atx in txs_list_for_desktop {
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
                for atx in txs_list_for_mobile {
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

async fn fetch_and_parse_transactions(api_base: &str, hashes: &[String], mut tx_cache: Signal<TxCache>) -> Vec<(String, ParsedTx)> {
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

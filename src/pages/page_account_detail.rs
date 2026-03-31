// pages/page_account_detail.rs
// =========================================
// Account detail page with transactions
// =========================================
use crate::api::types::{AccountFilters, AccountTx};
use crate::components::ui::{time_ago, transaction_hash};
use crate::logic::network::NetworkId;
use crate::logic::tx_cache::TxCache;
use crate::utils::parse_transaction::{parse_transaction, ParsedTx};
use dioxus::prelude::*;
use reqwest::Client;
use serde::Serialize;
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
pub fn AccountDetail(account_id: String, network: NetworkId) -> Element {
    let api_base = network.api_base_url();
    let network_val = network;

    // State
    let mut txs = use_signal(|| Vec::<AccountTx>::new());
    let mut parsed_txs = use_signal(|| Vec::<(String, ParsedTx)>::new());
    let mut loading = use_signal(|| true);
    let mut loading_more = use_signal(|| false);
    let mut resume_token = use_signal(|| Option::<String>::None);
    let mut has_more = use_signal(|| true);
    let mut txs_count = use_signal(|| 0u64);

    // Global cache for faster navigation
    let tx_cache = use_signal(|| TxCache::new());

    // Track current account to detect changes
    let mut current_account = use_signal(|| String::new());

    // Fetch data when account_id changes
    if current_account() != account_id {
        current_account.set(account_id.clone());
        loading.set(true);
        loading_more.set(false);
        resume_token.set(None);
        has_more.set(true);
        txs.set(Vec::new());
        parsed_txs.set(Vec::new());
        txs_count.set(0);

        let api_base = api_base.to_string();
        let account_id = account_id.clone();
        let mut tx_cache = tx_cache.clone();

        spawn(async move {
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
                        if let Some(account_txs) =
                            data.get("account_txs").and_then(|v| v.as_array())
                        {
                            let new_txs: Vec<AccountTx> = account_txs
                                .iter()
                                .filter_map(|v| serde_json::from_value(v.clone()).ok())
                                .collect();

                            let count = data.get("txs_count").and_then(|v| v.as_u64()).unwrap_or(0);
                            let token = data
                                .get("resume_token")
                                .and_then(|v| v.as_str())
                                .map(String::from);
                            let has_more_txs =
                                token.is_some() && new_txs.len() >= BATCH_SIZE as usize;

                            // Fetch full transaction details (with caching)
                            let hashes: Vec<String> =
                                new_txs.iter().map(|t| t.transaction_hash.clone()).collect();
                            let parsed =
                                fetch_and_parse_transactions(&api_base, &hashes, &mut tx_cache)
                                    .await;

                            txs.set(new_txs);
                            parsed_txs.set(parsed);
                            txs_count.set(count);
                            resume_token.set(token);
                            has_more.set(has_more_txs);
                        }
                    }
                }
                Err(_) => {}
            }
            loading.set(false);
        });
    }

    // Load more handler
    let account_id_for_load_more = account_id.clone();
    let load_more = move |_| {
        if loading_more() || !has_more() {
            return;
        }

        let api_base = api_base.to_string();
        let account_id = account_id_for_load_more.clone();
        let token = resume_token();
        loading_more.set(true);

        let mut tx_cache = tx_cache.clone();
        let mut txs_write = txs.clone();
        let mut parsed_txs_write = parsed_txs.clone();
        let mut resume_token_write = resume_token.clone();
        let mut has_more_write = has_more.clone();
        let mut loading_more_write = loading_more.clone();

        spawn(async move {
            let client = Client::new();
            let filters = AccountFilters::default();
            let params = AccountParams {
                account_id: &account_id,
                filters: &filters,
                resume_token: token.as_deref(),
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
                        if let Some(account_txs) =
                            data.get("account_txs").and_then(|v| v.as_array())
                        {
                            let new_txs: Vec<AccountTx> = account_txs
                                .iter()
                                .filter_map(|v| serde_json::from_value(v.clone()).ok())
                                .collect();

                            let token = data
                                .get("resume_token")
                                .and_then(|v| v.as_str())
                                .map(String::from);
                            let has_more_txs =
                                token.is_some() && new_txs.len() >= BATCH_SIZE as usize;

                            // Fetch full transaction details (with caching)
                            let hashes: Vec<String> =
                                new_txs.iter().map(|t| t.transaction_hash.clone()).collect();
                            let parsed =
                                fetch_and_parse_transactions(&api_base, &hashes, &mut tx_cache)
                                    .await;

                            txs_write.write().extend(new_txs);
                            parsed_txs_write.write().extend(parsed);
                            resume_token_write.set(token);
                            has_more_write.set(has_more_txs);
                        }
                    }
                }
                Err(_) => {}
            }
            loading_more_write.set(false);
        });
    };

    // Read state
    let loading_val = loading();
    let loading_more_val = loading_more();
    let has_more_val = has_more();
    let txs_list = txs();
    let parsed_list = parsed_txs();
    let txs_count_val = txs_count();
    let txs_list_empty = txs_list.is_empty();
    let account_id_display = account_id.clone();

    if loading_val {
        return rsx! {
            div { class: "empty-state", "Loading {account_id_display}..." }
        };
    }

    let parsed_map: std::collections::HashMap<String, ParsedTx> = parsed_list.into_iter().collect();
    let txs_list_for_display = txs_list.clone();

    rsx! {
        div {
            h1 { class: "mb-4 text-xl font-bold",
                "Account: "
                span { class: "font-mono text-base", "{account_id_display}" }
            }

            if txs_count_val > 0 {
                p { class: "mb-3 text-sm text-gray-600", "Transactions ({txs_count_val})" }
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
                        for atx in txs_list_for_display.iter() {
                            if let Some(parsed) = parsed_map.get(&atx.transaction_hash) {
                                tr {
                                    td {
                                        transaction_hash {
                                            hash: atx.transaction_hash.clone(),
                                            network: network_val,
                                        }
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
                for atx in txs_list_for_display.iter() {
                    if let Some(parsed) = parsed_map.get(&atx.transaction_hash) {
                        div {
                            div { class: "flex items-center justify-between gap-2 mb-1",
                                span { class: "font-mono text-xs",
                                    transaction_hash {
                                        hash: atx.transaction_hash.clone(),
                                        network: network_val,
                                    }
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

            // Load more button
            if has_more_val && !loading_val {
                div { class: "load-more-container",
                    button {
                        onclick: load_more,
                        disabled: loading_more_val,
                        class: "load-more-button",
                        if loading_more_val {
                            "Loading..."
                        } else {
                            "Load More"
                        }
                    }
                }
            }
        }
    }
}

async fn fetch_and_parse_transactions(
    api_base: &str,
    hashes: &[String],
    tx_cache: &mut Signal<TxCache>,
) -> Vec<(String, ParsedTx)> {
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
                                serde_json::from_value::<crate::api::types::TransactionDetail>(
                                    v.clone(),
                                )
                                .ok()
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

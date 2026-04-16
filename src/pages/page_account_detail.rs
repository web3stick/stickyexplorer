// pages/page_account_detail.rs
// =========================================
// Account detail page with transactions
// =========================================
use crate::api::client::ApiClient;
use crate::api::types::AccountFilters;
use crate::components::ui::{account_id as account_id_component, time_ago, transaction_hash};
use crate::ui_utils::network::NetworkId;
use crate::ui_utils::tx_cache::TxCache;
use crate::ui_utils::fetch_transactions::fetch_and_parse_transactions;
use crate::ui_utils::parse_transaction::ParsedTx;
use dioxus::prelude::*;
// =========================================

const BATCH_SIZE: u32 = 80;

#[component]
pub fn AccountDetail(account_id: String, network: NetworkId) -> Element {
    // State
    let mut txs = use_signal(|| Vec::new());
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

        let account_id_clone = account_id.clone();
        let network_clone = network.clone();
        let mut tx_cache_clone = tx_cache.clone();

        spawn(async move {
            let api_client = ApiClient::new(network_clone.api_base_url(), network_clone.as_str());
            let filters = AccountFilters::default();

            match api_client
                .get_account(&account_id_clone, &filters, None, Some(BATCH_SIZE))
                .await
            {
                Ok(data) => {
                    let new_txs = data.account_txs;
                    let count = data.txs_count;
                    let token = data.resume_token;
                    let has_more_txs = token.is_some() && new_txs.len() >= BATCH_SIZE as usize;

                    let hashes: Vec<String> =
                        new_txs.iter().map(|t| t.transaction_hash.clone()).collect();
                    let parsed =
                        fetch_and_parse_transactions(&api_client, &hashes, &mut tx_cache_clone)
                            .await;

                    txs.set(new_txs);
                    parsed_txs.set(parsed);
                    txs_count.set(count);
                    resume_token.set(token);
                    has_more.set(has_more_txs);
                }
                Err(_) => {}
            }
            loading.set(false);
        });
    }

    // Load more handler
    let account_id_for_jsx = account_id.clone();
    let load_more = move |_| {
        if loading_more() || !has_more() {
            return;
        }

        let token = resume_token();
        loading_more.set(true);

        let mut tx_cache = tx_cache.clone();
        let mut txs_write = txs.clone();
        let mut parsed_txs_write = parsed_txs.clone();
        let mut resume_token_write = resume_token.clone();
        let mut has_more_write = has_more.clone();
        let mut loading_more_write = loading_more.clone();

        spawn({
            let network_clone = network.clone();
            let account_id_clone = account_id.clone();
            async move {
                let api_client = ApiClient::new(network_clone.api_base_url(), network_clone.as_str());
                let filters = AccountFilters::default();

            match api_client
                .get_account(&account_id_clone, &filters, token.as_deref(), Some(BATCH_SIZE))
                .await
            {
                Ok(data) => {
                    let new_txs = data.account_txs;
                    let token = data.resume_token;
                    let has_more_txs = token.is_some() && new_txs.len() >= BATCH_SIZE as usize;

                    let hashes: Vec<String> =
                        new_txs.iter().map(|t| t.transaction_hash.clone()).collect();
                    let parsed =
                        fetch_and_parse_transactions(&api_client, &hashes, &mut tx_cache).await;

                    txs_write.write().extend(new_txs);
                    parsed_txs_write.write().extend(parsed);
                    resume_token_write.set(token);
                    has_more_write.set(has_more_txs);
                }
                Err(_) => {}
            }
            loading_more_write.set(false);
            }
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
    if loading_val {
        return rsx! {
            div { class: "empty-state", "Loading {account_id_for_jsx}..." }
        };
    }

    let parsed_map: std::collections::HashMap<String, ParsedTx> =
        parsed_list.into_iter().collect();
    let txs_list_for_display = txs_list.clone();

    rsx! {
        div {
            h1 { class: "mb-4 text-xl font-bold",
                "Account: "
                span { class: "font-mono text-base", "{account_id_for_jsx}" }
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
                                            network: network,
                                        }
                                    }
                                    td { class: "text-gray-500",
                                        time_ago { timestamp_ns: atx.tx_block_timestamp.clone() }
                                    }
                                    td {
                                        account_id_component {
                                            account_id: parsed.signer_id.clone(),
                                            network: network,
                                        }
                                    }
                                    td {
                                        account_id_component {
                                            account_id: parsed.receiver_id.clone(),
                                            network: network,
                                        }
                                    }
                                    td {
                                        if let Some(first_action) = parsed.actions.first() {
                                            span { class: "text-xs",
                                                "{first_action.action_type}"
                                                if let Some(ref method) = first_action.method_name {
                                                    "::{method}"
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
                                        network: network,
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
                                    account_id_component {
                                        account_id: parsed.signer_id.clone(),
                                        network: network,
                                    }
                                }
                                div {
                                    span { class: "text-gray-500 text-xs", "Receiver: " }
                                    account_id_component {
                                        account_id: parsed.receiver_id.clone(),
                                        network: network,
                                    }
                                }
                                div {
                                    span { class: "text-gray-500 text-xs", "Action: " }
                                    if let Some(first_action) = parsed.actions.first() {
                                        span { class: "text-xs",
                                            "{first_action.action_type}"
                                            if let Some(ref method) = first_action.method_name {
                                                "::{method}"
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
                            "LOADING..."
                        } else {
                            "LOAD MORE"
                        }
                    }
                }
            }
        }
    }
}
// =========================================
// copyright 2026 by sleet.near

// pages/page_account_detail.rs
// =========================================
// Account detail page with transactions
// =========================================
use dioxus::prelude::*;
use reqwest::Client;
use serde::Serialize;
use crate::api::types::{AccountFilters, AccountTx};
use crate::components::ui::{block_height, time_ago, transaction_hash};
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
            div { class: "hidden sm:block overflow-x-auto rounded-lg border border-gray-200 bg-white",
                table { class: "w-full text-sm",
                    thead {
                        tr { class: "border-b border-gray-200 bg-gray-50 text-left text-xs font-medium uppercase text-gray-500",
                            th { class: "px-4 py-3", "Tx Hash" }
                            th { class: "px-4 py-3", "Time" }
                            th { class: "px-4 py-3", "Block" }
                            th { class: "px-4 py-3", "Status" }
                        }
                    }
                    tbody {
                        for tx in txs_list_for_desktop {
                            tr { key: "{tx.transaction_hash}", class: "border-b border-gray-100 hover:bg-gray-50",
                                td { class: "px-4 py-3",
                                    transaction_hash { hash: tx.transaction_hash.clone() }
                                }
                                td { class: "px-4 py-3 text-gray-500",
                                    time_ago { timestamp_ns: tx.tx_block_timestamp.clone() }
                                }
                                td { class: "px-4 py-3",
                                    block_height { height: tx.tx_block_height }
                                }
                                td { class: "px-4 py-3",
                                    if tx.is_success {
                                        span { class: "text-green-600", "✓ Success" }
                                    } else {
                                        span { class: "text-red-600", "✗ Failed" }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Mobile cards
            div { class: "sm:hidden rounded-lg border border-gray-200 bg-white divide-y divide-gray-100",
                for tx in txs_list_for_mobile {
                    div { key: "{tx.transaction_hash}", class: "px-3 py-2.5",
                        div { class: "flex items-center justify-between gap-2 mb-1",
                            span { class: "font-mono text-xs",
                                transaction_hash { hash: tx.transaction_hash.clone() }
                            }
                            if tx.is_success {
                                span { class: "text-green-600 text-xs", "✓" }
                            } else {
                                span { class: "text-red-600 text-xs", "✗" }
                            }
                        }
                        div { class: "text-sm text-gray-500",
                            time_ago { timestamp_ns: tx.tx_block_timestamp.clone() }
                            " • Block "
                            block_height { height: tx.tx_block_height }
                        }
                    }
                }
            }

            if !loading_val && txs_list_empty {
                p { class: "py-8 text-center text-sm text-gray-500", "No transactions found" }
            }

            // Load more button
            if has_more_val {
                button {
                    onclick: load_more,
                    disabled: loading_more_val,
                    class: "mt-4 w-full py-2 px-4 bg-[#8CA2F5] text-white rounded-lg hover:bg-[#7a91e8] disabled:opacity-50",
                    if loading_more_val { "Loading..." } else { "Load More" }
                }
            }
        }
    }
}
// =========================================
// copyright 2026 by sleet.near

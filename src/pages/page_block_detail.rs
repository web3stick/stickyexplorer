// pages/page_block_detail.rs
// =========================================
// Block detail page
// =========================================
use dioxus::prelude::*;
use reqwest::Client;
use serde::Serialize;
use crate::api::types::{BlockDetailResponse, BlockTx};
use crate::components::ui::{account_id, block_height, block_hash, gas_amount, near_amount, time_ago, transaction_hash};
use crate::logic::network::get_stored_network_id;
// =========================================

#[derive(Clone, Serialize)]
struct BlockParams {
    block_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    with_transactions: Option<bool>,
}

#[component]
pub fn BlockDetail(block_id: String) -> Element {
    let mut block = use_signal(|| Option::<BlockDetailResponse>::None);
    let mut loading = use_signal(|| true);
    let mut error = use_signal(|| Option::<String>::None);
    let mut visible_count = use_signal(|| 40usize);

    let network_id = get_stored_network_id();
    let api_base = network_id.api_base_url();

    use_effect(move || {
        let api_base = api_base.to_string();
        let block_id = block_id.clone();
        spawn(async move {
            loading.set(true);
            error.set(None);
            
            let client = Client::new();
            
            // Determine if block_id is a number (height) or string (hash)
            let block_id_str = if block_id.parse::<u64>().is_ok() {
                block_id.clone()
            } else {
                block_id.clone()
            };
            
            let params = BlockParams {
                block_id: block_id_str,
                with_transactions: Some(true),
            };

            match client
                .post(format!("{}/v0/block", api_base))
                .json(&params)
                .send()
                .await
            {
                Ok(resp) => {
                    if let Ok(data) = resp.json::<serde_json::Value>().await {
                        if let Some(_block_data) = data.get("block") {
                            if let Ok(block_detail) = serde_json::from_value::<BlockDetailResponse>(data.clone()) {
                                block.set(Some(block_detail));
                            }
                        } else {
                            error.set(Some("Block not found".to_string()));
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
        visible_count.set(visible_count() + 40);
    };

    if let Some(err) = error() {
        return rsx! {
            p { class: "text-red-600", "{err}" }
        };
    }

    if loading() || block().is_none() {
        return rsx! {
            p { class: "text-gray-500", "Loading block..." }
        };
    }

    let block_data = block().unwrap();
    let b = &block_data.block;
    let txs = &block_data.block_txs;
    let visible_count_val = visible_count();
    let has_more = visible_count_val < txs.len();
    
    // Clone all values needed for rsx before moving
    let block_height_val = b.block_height;
    let block_hash_val = b.block_hash.clone();
    let block_timestamp_val = b.block_timestamp.clone();
    let author_id_val = b.author_id.clone();
    let epoch_id_val = b.epoch_id.clone();
    let prev_block_height_val = b.prev_block_height;
    let prev_block_hash_val = b.prev_block_hash.clone();
    let num_transactions_val = b.num_transactions;
    let num_receipts_val = b.num_receipts;
    let gas_burnt_val = b.gas_burnt.parse().unwrap_or(0);
    let gas_price_val = b.gas_price.clone();
    let tokens_burnt_val = b.tokens_burnt.clone();
    let chunks_included_val = b.chunks_included;
    let protocol_version_val = b.protocol_version;
    
    let txs_list: Vec<BlockTx> = txs.iter().take(visible_count_val).cloned().collect();
    let txs_list_for_desktop = txs_list.clone();
    let txs_list_for_mobile = txs_list.clone();
    let has_more_val = has_more;

    rsx! {
        div {
            h1 { class: "mb-4 text-xl font-bold",
                "Block #"
                Link {
                    to: format!("/block/{block_height_val}"),
                    class: "hover:underline",
                    "{block_height_val}"
                }
            }

            div { class: "mb-6 rounded-lg border border-gray-200 bg-white text-sm",
                dl { class: "grid gap-px sm:grid-cols-2 [&>div]:flex [&>div]:min-w-0 [&>div]:gap-2 [&>div]:border-b [&>div]:border-gray-100 [&>div]:px-4 [&>div]:py-3 [&>div:last-child]:border-b-0",
                    // Hash
                    div {
                        dt { class: "shrink-0 text-gray-500", "Hash" }
                        dd { class: "min-w-0 truncate",
                            block_hash { hash: block_hash_val.clone() }
                        }
                    }
                    // Timestamp
                    div {
                        dt { class: "shrink-0 text-gray-500", "Timestamp" }
                        dd {
                            time_ago { timestamp_ns: block_timestamp_val.clone() }
                        }
                    }
                    // Author
                    div {
                        dt { class: "shrink-0 text-gray-500", "Author" }
                        dd {
                            account_id { account_id: author_id_val.clone() }
                        }
                    }
                    // Epoch ID
                    div {
                        dt { class: "shrink-0 text-gray-500", "Epoch ID" }
                        dd { class: "min-w-0 truncate font-mono text-xs", "{epoch_id_val}" }
                    }
                    // Prev Block
                    div {
                        dt { class: "shrink-0 text-gray-500", "Prev Block" }
                        dd {
                            if let Some(prev_height) = prev_block_height_val {
                                block_height { height: prev_height }
                            } else {
                                block_hash { hash: prev_block_hash_val.clone() }
                            }
                        }
                    }
                    // Transactions
                    div {
                        dt { class: "shrink-0 text-gray-500", "Transactions" }
                        dd { "{num_transactions_val}" }
                    }
                    // Receipts
                    div {
                        dt { class: "shrink-0 text-gray-500", "Receipts" }
                        dd { "{num_receipts_val}" }
                    }
                    // Gas Used
                    div {
                        dt { class: "shrink-0 text-gray-500", "Gas Used" }
                        dd {
                            gas_amount { gas: gas_burnt_val }
                        }
                    }
                    // Gas Price
                    div {
                        dt { class: "shrink-0 text-gray-500", "Gas Price" }
                        dd {
                            near_amount { yocto_near: gas_price_val.clone() }
                        }
                    }
                    // Tokens Burnt
                    div {
                        dt { class: "shrink-0 text-gray-500", "Tokens Burnt" }
                        dd {
                            near_amount { yocto_near: tokens_burnt_val.clone(), show_price: true }
                        }
                    }
                    // Chunks
                    div {
                        dt { class: "shrink-0 text-gray-500", "Chunks" }
                        dd { "{chunks_included_val}" }
                    }
                    // Protocol Version
                    div {
                        dt { class: "shrink-0 text-gray-500", "Protocol Version" }
                        dd { "{protocol_version_val}" }
                    }
                }
            }

            // Transactions
            if !txs_list.is_empty() {
                div {
                    p { class: "mb-3 text-sm text-gray-600",
                        "Transactions ({num_transactions_val})"
                    }
                    
                    // Desktop table
                    div { class: "hidden sm:block overflow-x-auto rounded-lg border border-gray-200 bg-white",
                        table { class: "w-full text-sm",
                            thead {
                                tr { class: "border-b border-gray-200 bg-gray-50 text-left text-xs font-medium uppercase text-gray-500",
                                    th { class: "px-4 py-3", "Tx Hash" }
                                    th { class: "px-4 py-3", "Time" }
                                    th { class: "px-4 py-3", "Signer" }
                                    th { class: "px-4 py-3", "Receiver" }
                                    th { class: "px-4 py-3 text-right", "Gas" }
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
                                            account_id { account_id: tx.signer_id.clone() }
                                        }
                                        td { class: "px-4 py-3",
                                            account_id { account_id: tx.receiver_id.clone() }
                                        }
                                        td { class: "px-4 py-3 text-right",
                                            gas_amount { gas: tx.gas_burnt }
                                        }
                                        td { class: "px-4 py-3",
                                            if tx.is_success {
                                                span { class: "text-green-600", "✓" }
                                            } else {
                                                span { class: "text-red-600", "✗" }
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
                                }
                                div { class: "text-sm",
                                    account_id { account_id: tx.signer_id.clone() }
                                    " → "
                                    account_id { account_id: tx.receiver_id.clone() }
                                }
                            }
                        }
                    }

                    // Load more button
                    if has_more_val {
                        button {
                            onclick: load_more,
                            class: "mt-4 w-full py-2 px-4 bg-[#8CA2F5] text-white rounded-lg hover:bg-[#7a91e8]",
                            "Show More"
                        }
                    }
                }
            }
        }
    }
}
// =========================================
// copyright 2026 by sleet.near

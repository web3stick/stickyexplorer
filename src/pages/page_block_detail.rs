// pages/page_block_detail.rs
// =========================================
// Block detail page
// =========================================
use crate::api::client::{ApiClient, BlockId};
use crate::api::types::{BlockDetailResponse, BlockTx};
use crate::components::ui::{
    account_id, block_hash, block_height, gas_amount, near_amount, time_ago, transaction_hash,
};
use crate::logic::network::NetworkId;
use dioxus::prelude::*;
// =========================================

#[component]
pub fn BlockDetail(block_id: String, network: NetworkId) -> Element {
    // State
    let mut block = use_signal(|| Option::<BlockDetailResponse>::None);
    let mut loading = use_signal(|| true);
    let mut error = use_signal(|| Option::<String>::None);
    let mut visible_count = use_signal(|| 40usize);

    // Track current block_id to detect changes
    let mut current_block_id = use_signal(|| String::new());

    // Fetch data when block_id changes
    if current_block_id() != block_id {
        current_block_id.set(block_id.clone());
        loading.set(true);
        error.set(None);
        visible_count.set(40);
        block.set(None);

        let block_id_clone = block_id.clone();
        let network_clone = network.clone();

        spawn(async move {
            let api_client = ApiClient::new(network_clone.api_base_url(), network_clone.as_str());
            let block_identifier = if let Ok(height) = block_id_clone.parse::<u64>() {
                BlockId::Height(height)
            } else {
                BlockId::Hash(block_id_clone)
            };

            match api_client.get_block(block_identifier, true).await {
                Ok(block_detail) => {
                    block.set(Some(block_detail));
                }
                Err(e) => {
                    error.set(Some(e));
                }
            }
            loading.set(false);
        });
    }

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
    let network_val = network;

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

            div { class: "detail-card",
                dl {
                    // Hash
                    div {
                        dt { "Hash" }
                        dd {
                            block_hash { hash: block_hash_val.clone() }
                        }
                    }
                    // Timestamp
                    div {
                        dt { "Timestamp" }
                        dd {
                            time_ago { timestamp_ns: block_timestamp_val.clone() }
                        }
                    }
                    // Author
                    div {
                        dt { "Author" }
                        dd {
                            account_id {
                                account_id: author_id_val.clone(),
                                network: network_val,
                            }
                        }
                    }
                    // Epoch ID
                    div {
                        dt { "Epoch ID" }
                        dd { class: "font-mono text-xs truncate", "{epoch_id_val}" }
                    }
                    // Prev Block
                    div {
                        dt { "Prev Block" }
                        dd {
                            if let Some(prev_height) = prev_block_height_val {
                                block_height { height: prev_height, network: network_val }
                            } else {
                                block_hash { hash: prev_block_hash_val.clone() }
                            }
                        }
                    }
                    // Transactions
                    div {
                        dt { "Transactions" }
                        dd { "{num_transactions_val}" }
                    }
                    // Receipts
                    div {
                        dt { "Receipts" }
                        dd { "{num_receipts_val}" }
                    }
                    // Gas Used
                    div {
                        dt { "Gas Used" }
                        dd {
                            gas_amount { gas: gas_burnt_val }
                        }
                    }
                    // Gas Price
                    div {
                        dt { "Gas Price" }
                        dd {
                            near_amount { yocto_near: gas_price_val.clone() }
                        }
                    }
                    // Tokens Burnt
                    div {
                        dt { "Tokens Burnt" }
                        dd {
                            near_amount {
                                yocto_near: tokens_burnt_val.clone(),
                                show_price: true,
                            }
                        }
                    }
                    // Chunks
                    div {
                        dt { "Chunks" }
                        dd { "{chunks_included_val}" }
                    }
                    // Protocol Version
                    div {
                        dt { "Protocol Version" }
                        dd { "{protocol_version_val}" }
                    }
                }
            }

            // Transactions
            if !txs_list.is_empty() {
                div {
                    h2 { class: "section-heading", "Transactions ({num_transactions_val})" }

                    // Desktop table
                    div { class: "table-container",
                        table {
                            thead {
                                tr {
                                    th { "Tx Hash" }
                                    th { "Time" }
                                    th { "Signer" }
                                    th { "Receiver" }
                                    th { class: "text-right", "Gas" }
                                    th { "Status" }
                                }
                            }
                            tbody {
                                for tx in txs_list_for_desktop {
                                    tr {
                                        td {
                                            transaction_hash {
                                                hash: tx.transaction_hash.clone(),
                                                network: network_val,
                                            }
                                        }
                                        td { class: "text-gray-500",
                                            time_ago { timestamp_ns: tx.tx_block_timestamp.clone() }
                                        }
                                        td {
                                            account_id {
                                                account_id: tx.signer_id.clone(),
                                                network: network_val,
                                            }
                                        }
                                        td {
                                            account_id {
                                                account_id: tx.receiver_id.clone(),
                                                network: network_val,
                                            }
                                        }
                                        td { class: "text-right",
                                            gas_amount { gas: tx.gas_burnt }
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
                                    span { class: "font-mono text-xs truncate",
                                        transaction_hash {
                                            hash: tx.transaction_hash.clone(),
                                            network: network_val,
                                        }
                                    }
                                    if tx.is_success {
                                        span { class: "status-success text-xs", "✓" }
                                    } else {
                                        span { class: "status-failed text-xs", "✗" }
                                    }
                                }
                                div { class: "text-sm text-gray-500",
                                    time_ago { timestamp_ns: tx.tx_block_timestamp.clone() }
                                }
                                div { class: "text-sm",
                                    account_id {
                                        account_id: tx.signer_id.clone(),
                                        network: network_val,
                                    }
                                    " → "
                                    account_id {
                                        account_id: tx.receiver_id.clone(),
                                        network: network_val,
                                    }
                                }
                            }
                        }
                    }

                    // Load more button
                    if has_more_val {
                        div { class: "load-more-container",
                            button { onclick: load_more, class: "load-more-button", "Show More" }
                        }
                    }
                }
            }
        }
    }
}
// =========================================
// copyright 2026 by sleet.near

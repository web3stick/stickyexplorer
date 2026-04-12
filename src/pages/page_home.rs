// pages/page_home.rs
// =========================================
// Home page - Latest blocks list with infinite scroll
// =========================================
use crate::api::client::ApiClient;
use crate::api::types::BlockHeader;
use crate::components::ui::{account_id, block_height, gas_amount, time_ago};
use crate::logic::network::get_stored_network_id;
use dioxus::prelude::*;
// =========================================

const BATCH_SIZE: u32 = 80;

#[component]
pub fn Home() -> Element {
    let mut blocks = use_signal(|| Vec::<BlockHeader>::new());
    let mut loading = use_signal(|| true);
    let mut error = use_signal(|| Option::<String>::None);
    let mut resume_token = use_signal(|| Option::<u64>::None);
    let mut has_more = use_signal(|| true);
    let mut loading_more = use_signal(|| false);
    let network_id = use_signal(|| get_stored_network_id());

    // Initial load
    use_effect(move || {
        let net = network_id();
        spawn(async move {
            loading.set(true);
            error.set(None);

            let api_client = ApiClient::new(net.api_base_url(), net.as_str());

            match api_client
                .get_blocks(Some(BATCH_SIZE), Some(true), None, None)
                .await
            {
                Ok(data) => {
                    let block_array = data.blocks;
                    if !block_array.is_empty() {
                        if let Some(last) = block_array.last() {
                            resume_token.set(Some(last.block_height.saturating_sub(1)));
                        }
                    } else {
                        has_more.set(false);
                    }
                    blocks.set(block_array);
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

        let net = network_id();
        let token = resume_token();
        loading_more.set(true);

        spawn(async move {
            let api_client = ApiClient::new(net.api_base_url(), net.as_str());

            match api_client
                .get_blocks(Some(BATCH_SIZE), Some(true), token, None)
                .await
            {
                Ok(data) => {
                    let new_blocks = data.blocks;
                    if new_blocks.is_empty() || new_blocks.len() < BATCH_SIZE as usize {
                        has_more.set(false);
                    }
                    if let Some(last) = new_blocks.last() {
                        resume_token.set(Some(last.block_height.saturating_sub(1)));
                    }
                    blocks.write().extend(new_blocks);
                }
                Err(_) => {
                    has_more.set(false);
                }
            }
            loading_more.set(false);
        });
    };

    if let Some(err) = error() {
        return rsx! {
            p { class: "text-red-600", "Error loading blocks: {err}" }
        };
    }

    // Performance optimization: snapshot blocks signal once to avoid repeated
    // signal reads in the render path. This reduces overhead for large lists.
    let blocks_snapshot = blocks();
    let blocks_empty = blocks_snapshot.is_empty();

    rsx! {
        div {
            h1 { class: "mb-4 text-xl font-bold", "Latest Blocks" }

            // Desktop table (mobile cards handled via CSS responsive design)
            if !blocks_empty {
                div { class: "table-container",
                    table {
                        thead {
                            tr {
                                th { "Height" }
                                th { "Time" }
                                th { "Author" }
                                th { class: "text-right", "Txns" }
                                th { class: "text-right", "Receipts" }
                                th { class: "text-right", "Gas Used" }
                            }
                        }
                        tbody {
                            for block in blocks_snapshot.iter() {
                                tr { key: "{block.block_hash}",
                                    td {
                                        span { class: "font-medium",
                                            block_height { height: block.block_height, network: network_id() }
                                        }
                                    }
                                    td { class: "text-gray-500",
                                        time_ago { timestamp_ns: block.block_timestamp.clone() }
                                    }
                                    td {
                                        account_id { account_id: block.author_id.clone(), network: network_id() }
                                    }
                                    td { class: "text-right", "{block.num_transactions}" }
                                    td { class: "text-right", "{block.num_receipts}" }
                                    td { class: "text-right",
                                        gas_amount { gas: block.gas_burnt.parse().unwrap_or(0) }
                                    }
                                }
                            }
                        }
                    }
                }

                // Mobile cards
                div { class: "mobile-cards",
                    for block in blocks_snapshot.iter() {
                        div { key: "{block.block_hash}",
                            div { class: "flex items-center justify-between gap-2 mb-1",
                                span { class: "font-medium",
                                    block_height { height: block.block_height, network: network_id() }
                                }
                                span { class: "text-xs text-gray-500",
                                    time_ago { timestamp_ns: block.block_timestamp.clone() }
                                }
                            }
                            div { class: "text-sm mb-1",
                                span { class: "text-gray-500 text-xs", "Author: " }
                                account_id { account_id: block.author_id.clone(), network: network_id() }
                            }
                            div { class: "grid grid-cols-3 gap-2 text-xs",
                                div {
                                    span { class: "text-gray-500", "Txns: " }
                                    "{block.num_transactions}"
                                }
                                div {
                                    span { class: "text-gray-500", "Receipts: " }
                                    "{block.num_receipts}"
                                }
                                div {
                                    span { class: "text-gray-500", "Gas: " }
                                    gas_amount { gas: block.gas_burnt.parse().unwrap_or(0) }
                                }
                            }
                        }
                    }
                }
            }

            if !loading() && blocks_empty {
                p { class: "empty-state", "No blocks available" }
            }

            // Load more button - centered
            if has_more() {
                div { class: "load-more-container",
                    button {
                        onclick: load_more,
                        disabled: loading_more(),
                        class: "load-more-button",
                        if loading_more() {
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
// =========================================
// copyright 2026 by sleet.near

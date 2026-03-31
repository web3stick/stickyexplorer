// pages/page_home.rs
// =========================================
// Home page - Latest blocks list with infinite scroll
// =========================================
use dioxus::prelude::*;
use reqwest::Client;
use serde::Serialize;
use crate::api::types::BlockHeader;
use crate::components::ui::{account_id, block_height, gas_amount, time_ago};
use crate::logic::network::get_stored_network_id;
// =========================================

const BATCH_SIZE: u32 = 80;

#[derive(Clone, Serialize)]
struct BlocksParams {
    limit: u32,
    desc: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_block_height: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_block_height: Option<u64>,
}

#[component]
pub fn Home() -> Element {
    let mut blocks = use_signal(|| Vec::<BlockHeader>::new());
    let mut loading = use_signal(|| true);
    let mut error = use_signal(|| Option::<String>::None);
    let mut resume_token = use_signal(|| Option::<u64>::None);
    let mut has_more = use_signal(|| true);
    let mut loading_more = use_signal(|| false);

    let network_id = get_stored_network_id();
    let api_base = network_id.api_base_url();

    // Initial load
    use_effect(move || {
        let api_base = api_base.to_string();
        spawn(async move {
            loading.set(true);
            error.set(None);
            
            let client = Client::new();
            let params = BlocksParams {
                limit: BATCH_SIZE,
                desc: true,
                to_block_height: None,
                from_block_height: None,
            };

            match client
                .post(format!("{}/v0/blocks", api_base))
                .json(&params)
                .send()
                .await
            {
                Ok(resp) => {
                    if let Ok(data) = resp.json::<serde_json::Value>().await {
                        if let Some(block_array) = data.get("blocks").and_then(|v| v.as_array()) {
                            let new_blocks: Vec<BlockHeader> = block_array
                                .iter()
                                .filter_map(|v| serde_json::from_value(v.clone()).ok())
                                .collect();
                            
                            if !new_blocks.is_empty() {
                                if let Some(last) = new_blocks.last() {
                                    resume_token.set(Some(last.block_height.saturating_sub(1)));
                                }
                            } else {
                                has_more.set(false);
                            }
                            blocks.set(new_blocks);
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
        let token = resume_token();
        loading_more.set(true);

        spawn(async move {
            let client = Client::new();
            let params = BlocksParams {
                limit: BATCH_SIZE,
                desc: true,
                to_block_height: token,
                from_block_height: None,
            };

            if let Ok(resp) = client
                .post(format!("{}/v0/blocks", api_base))
                .json(&params)
                .send()
                .await
            {
                if let Ok(data) = resp.json::<serde_json::Value>().await {
                    if let Some(block_array) = data.get("blocks").and_then(|v| v.as_array()) {
                        let new_blocks: Vec<BlockHeader> = block_array
                            .iter()
                            .filter_map(|v| serde_json::from_value(v.clone()).ok())
                            .collect();
                        
                        if new_blocks.is_empty() || new_blocks.len() < BATCH_SIZE as usize {
                            has_more.set(false);
                        }
                        
                        if let Some(last) = new_blocks.last() {
                            resume_token.set(Some(last.block_height.saturating_sub(1)));
                        }
                        
                        blocks.write().extend(new_blocks);
                    }
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

    rsx! {
        div {
            h1 { class: "mb-4 text-xl font-bold", "Latest Blocks" }
            
            // Desktop table
            div { class: "hidden sm:block overflow-x-auto rounded-lg border border-gray-200 bg-white",
                table { class: "w-full text-sm",
                    thead {
                        tr { class: "border-b border-gray-200 bg-gray-50 text-left text-xs font-medium uppercase text-gray-500",
                            th { class: "px-4 py-3", "Height" }
                            th { class: "px-4 py-3", "Time" }
                            th { class: "px-4 py-3", "Author" }
                            th { class: "px-4 py-3 text-right", "Txns" }
                            th { class: "px-4 py-3 text-right", "Receipts" }
                            th { class: "px-4 py-3 text-right", "Gas Used" }
                        }
                    }
                    tbody {
                        for block in blocks() {
                            tr { key: "{block.block_height}", class: "border-b border-gray-100 hover:bg-gray-50",
                                td { class: "px-4 py-3",
                                    span { class: "font-medium",
                                        block_height { height: block.block_height }
                                    }
                                }
                                td { class: "px-4 py-3 text-gray-500",
                                    time_ago { timestamp_ns: block.block_timestamp.clone() }
                                }
                                td { class: "px-4 py-3",
                                    account_id { account_id: block.author_id.clone() }
                                }
                                td { class: "px-4 py-3 text-right", "{block.num_transactions}" }
                                td { class: "px-4 py-3 text-right", "{block.num_receipts}" }
                                td { class: "px-4 py-3 text-right",
                                    gas_amount { gas: block.gas_burnt.parse().unwrap_or(0) }
                                }
                            }
                        }
                    }
                }
            }

            // Mobile cards
            div { class: "sm:hidden rounded-lg border border-gray-200 bg-white divide-y divide-gray-100",
                for block in blocks() {
                    div { key: "{block.block_height}", class: "px-3 py-2.5",
                        div { class: "flex items-center justify-between gap-2 mb-0.5",
                            span { class: "font-medium text-sm",
                                block_height { height: block.block_height }
                            }
                            span { class: "text-xs text-gray-500 shrink-0",
                                time_ago { timestamp_ns: block.block_timestamp.clone() }
                            }
                        }
                        div { class: "flex items-center justify-between gap-2",
                            account_id { account_id: block.author_id.clone() }
                            span { class: "text-xs text-gray-500 shrink-0", "{block.num_transactions} txns" }
                        }
                    }
                }
            }

            if !loading() && blocks().is_empty() {
                p { class: "py-8 text-center text-sm text-gray-500", "No blocks available" }
            }

            // Load more button
            if has_more() {
                button {
                    onclick: load_more,
                    disabled: loading_more(),
                    class: "mt-4 w-full py-2 px-4 bg-[#8CA2F5] text-white rounded-lg hover:bg-[#7a91e8] disabled:opacity-50",
                    if loading_more() { "Loading..." } else { "Load More" }
                }
            }
        }
    }
}
// =========================================
// copyright 2026 by sleet.near

// pages/page_tx_detail.rs
// =========================================
// Transaction detail page
// =========================================
use dioxus::prelude::*;
use reqwest::Client;
use serde::Serialize;
use crate::api::types::TransactionDetail;
use crate::components::ui::{account_id, block_height, gas_amount, time_ago, transaction_hash};
use crate::logic::network::get_stored_network_id;
use crate::utils::parse_transaction::{parse_transaction, ParsedTx};
// =========================================

#[derive(Clone, Serialize)]
struct TxParams {
    tx_hashes: Vec<String>,
}

#[component]
pub fn TxDetail(tx_hash: String) -> Element {
    let mut tx = use_signal(|| Option::<TransactionDetail>::None);
    let mut parsed_tx = use_signal(|| Option::<ParsedTx>::None);
    let mut loading = use_signal(|| true);
    let mut error = use_signal(|| Option::<String>::None);

    let network_id = get_stored_network_id();
    let api_base = network_id.api_base_url();

    use_effect(move || {
        let api_base = api_base.to_string();
        let tx_hash = tx_hash.clone();
        spawn(async move {
            loading.set(true);
            error.set(None);
            
            let client = Client::new();
            let params = TxParams {
                tx_hashes: vec![tx_hash],
            };

            match client
                .post(format!("{}/v0/transactions", api_base))
                .json(&params)
                .send()
                .await
            {
                Ok(resp) => {
                    if let Ok(data) = resp.json::<serde_json::Value>().await {
                        if let Some(transactions) = data.get("transactions").and_then(|v| v.as_array()) {
                            if let Some(first_tx) = transactions.first() {
                                if let Ok(tx_detail) = serde_json::from_value::<TransactionDetail>(first_tx.clone()) {
                                    let parsed = parse_transaction(&tx_detail);
                                    parsed_tx.set(Some(parsed));
                                    tx.set(Some(tx_detail));
                                }
                            } else {
                                error.set(Some("Transaction not found".to_string()));
                            }
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

    if let Some(err) = error() {
        return rsx! {
            p { class: "text-red-600", "{err}" }
        };
    }

    if loading() || parsed_tx().is_none() {
        return rsx! {
            p { class: "text-gray-500", "Loading transaction..." }
        };
    }

    let ptx = parsed_tx().unwrap();

    rsx! {
        div {
            h1 { class: "mb-4 text-xl font-bold", "Transaction" }

            div { class: "mb-6 rounded-lg border border-gray-200 bg-white text-sm",
                dl { class: "grid gap-px sm:grid-cols-2 [&>div]:flex [&>div]:items-center [&>div]:gap-2 [&>div]:border-b [&>div]:border-gray-100 [&>div]:px-4 [&>div]:py-2 [&>div:last-child]:border-b-0",
                    // Hash
                    div { class: "sm:col-span-2",
                        dt { class: "shrink-0 text-gray-500", "Hash" }
                        dd { class: "flex flex-1 min-w-0 items-center justify-between gap-2",
                            span { class: "break-all",
                                transaction_hash { hash: ptx.hash.clone(), truncate: false }
                            }
                            span {
                                if let Some(success) = ptx.is_success {
                                    if success {
                                        span { class: "text-green-600", "✓" }
                                    } else {
                                        span { class: "text-red-600", "✗" }
                                    }
                                } else {
                                    span { class: "text-yellow-500", "⏳" }
                                }
                            }
                        }
                    }
                    // Signer
                    div {
                        dt { class: "shrink-0 text-gray-500", "Signer" }
                        dd {
                            account_id { account_id: ptx.signer_id.clone() }
                        }
                    }
                    // Receiver
                    div {
                        dt { class: "shrink-0 text-gray-500", "Receiver" }
                        dd {
                            account_id { account_id: ptx.receiver_id.clone() }
                        }
                    }
                    // Block
                    div {
                        dt { class: "shrink-0 text-gray-500", "Block" }
                        dd {
                            block_height { height: ptx.block_height }
                        }
                    }
                    // Time
                    div {
                        dt { class: "shrink-0 text-gray-500", "Time" }
                        dd {
                            time_ago { timestamp_ns: ptx.timestamp.clone() }
                        }
                    }
                    // Gas
                    div {
                        dt { class: "shrink-0 text-gray-500", "Gas Used" }
                        dd {
                            gas_amount { gas: ptx.gas_burnt }
                        }
                    }
                }
            }

            // Actions
            if !ptx.actions.is_empty() {
                div { class: "mb-6 rounded-lg border border-gray-200 bg-white text-sm",
                    div { class: "border-b border-gray-100 px-4 py-2",
                        h2 { class: "text-xs font-medium uppercase text-gray-500", "Actions" }
                    }
                    div { class: "px-4 py-3",
                        for (i, action) in ptx.actions.iter().enumerate() {
                            div { key: "{i}", class: "font-mono text-xs py-1",
                                "{action.action_type}"
                                if let Some(ref method) = action.method_name {
                                    span { class: "text-gray-500", "::{method}" }
                                }
                            }
                        }
                    }
                }
            }

            // Transfers
            if !ptx.transfers.is_empty() || !ptx.nft_transfers.is_empty() {
                div { class: "mb-6 rounded-lg border border-gray-200 bg-white text-sm",
                    div { class: "border-b border-gray-100 px-4 py-2",
                        h2 { class: "text-xs font-medium uppercase text-gray-500", "Transfers" }
                    }
                    div { class: "px-4 py-3 flex flex-col gap-1",
                        for transfer in ptx.transfers {
                            div { key: "ft-{transfer.amount}", class: "text-xs",
                                if let Some(from) = &transfer.from {
                                    span { "{from} → " }
                                }
                                if let Some(to) = &transfer.to {
                                    span { "{to}: " }
                                }
                                span { class: "font-mono", "{transfer.amount}" }
                                span { class: "text-gray-500",
                                    match transfer.token_type {
                                        crate::utils::parse_transaction::TokenType::Near => " NEAR",
                                        crate::utils::parse_transaction::TokenType::Nep141 => " (FT)",
                                        crate::utils::parse_transaction::TokenType::Nep245 => " (MT)",
                                    }
                                }
                            }
                        }
                        for nft_transfer in ptx.nft_transfers {
                            div { key: "nft-{nft_transfer.token_id}", class: "text-xs",
                                if let Some(from) = &nft_transfer.from {
                                    span { "{from} → " }
                                }
                                if let Some(to) = &nft_transfer.to {
                                    span { "{to}: " }
                                }
                                span { class: "font-mono", "Token #{nft_transfer.token_id}" }
                                span { class: "text-gray-500", " (NFT)" }
                            }
                        }
                    }
                }
            }

            // Receipts
            if !ptx.receipts.is_empty() {
                div { class: "mb-6",
                    h2 { class: "mb-3 text-lg font-semibold", "Receipts ({ptx.receipts.len()})" }
                    div { class: "space-y-3",
                        for receipt in ptx.receipts {
                            div { key: "{receipt.receipt.receipt_id}", class: "rounded-lg border border-gray-200 bg-white p-4 text-sm",
                                div { class: "grid grid-cols-2 gap-2",
                                    div {
                                        span { class: "text-gray-500", "From: " }
                                        account_id { account_id: receipt.receipt.predecessor_id.clone() }
                                    }
                                    div {
                                        span { class: "text-gray-500", "To: " }
                                        account_id { account_id: receipt.receipt.receiver_id.clone() }
                                    }
                                    div {
                                        span { class: "text-gray-500", "Gas: " }
                                        gas_amount { gas: receipt.execution_outcome.outcome.gas_burnt }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
// =========================================
// copyright 2026 by sleet.near

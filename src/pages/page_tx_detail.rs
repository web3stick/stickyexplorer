// pages/page_tx_detail.rs
// =========================================
// Transaction detail page
// =========================================
use crate::api::client::ApiClient;
use crate::api::types::TransactionDetail;
use crate::components::ui::{account_id, block_height, gas_amount, time_ago, transaction_hash};
use crate::components::widgets::{get_matching_widgets, DefaultWidget, WidgetType};
use crate::ui_utils::network::NetworkId;
use crate::ui_utils::parse_transaction::{parse_transaction, ParsedTx};
use dioxus::prelude::*;
// =========================================

#[component]
pub fn TxDetail(tx_hash: String, network: NetworkId) -> Element {
    // State
    let mut tx = use_signal(|| Option::<TransactionDetail>::None);
    let mut parsed_tx = use_signal(|| Option::<ParsedTx>::None);
    let mut loading = use_signal(|| true);
    let mut error = use_signal(|| Option::<String>::None);

    // Track current tx_hash to detect changes
    let mut current_tx_hash = use_signal(|| String::new());

    // Fetch data when tx_hash changes
    if current_tx_hash() != tx_hash {
        current_tx_hash.set(tx_hash.clone());
        loading.set(true);
        error.set(None);
        tx.set(None);
        parsed_tx.set(None);

        let tx_hash_clone = tx_hash.clone();
        let network_clone = network.clone();

        spawn(async move {
            let api_client = ApiClient::new(network_clone.api_base_url(), network_clone.as_str());

            match api_client.get_transactions(vec![tx_hash_clone]).await {
                Ok(data) => {
                    if let Some(first_tx) = data.transactions.first() {
                        let parsed = parse_transaction(first_tx);
                        parsed_tx.set(Some(parsed.clone()));
                        tx.set(Some(first_tx.clone()));
                    } else {
                        error.set(Some("Transaction not found".to_string()));
                    }
                }
                Err(e) => {
                    error.set(Some(e));
                }
            }
            loading.set(false);
        });
    }

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
    let tx_detail = tx().unwrap();
    let widgets = get_matching_widgets(&tx_detail);

    // Explanation widgets only (utility widgets like raw JSON are rendered separately)
    let explanation_widgets: Vec<_> = widgets
        .iter()
        .filter(|w| w.widget_type == WidgetType::Explanation)
        .collect();

    rsx! {
        div {
            h1 { class: "mb-4 text-xl font-bold", "Transaction" }

            div { class: "detail-card",
                dl {
                    // Hash
                    div { class: "sm:col-span-2",
                        dt { "Hash" }
                        dd { class: "break-all",
                            transaction_hash {
                                hash: ptx.hash.clone(),
                                truncate: false,
                                network: network,
                            }
                        }
                    }
                    // Signer
                    div {
                        dt { "Signer" }
                        dd {
                            account_id {
                                account_id: ptx.signer_id.clone(),
                                network: network,
                            }
                        }
                    }
                    // Receiver
                    div {
                        dt { "Receiver" }
                        dd {
                            account_id {
                                account_id: ptx.receiver_id.clone(),
                                network: network,
                            }
                        }
                    }
                    // Block
                    div {
                        dt { "Block" }
                        dd {
                            block_height {
                                height: ptx.block_height,
                                network: network,
                            }
                        }
                    }
                    // Time
                    div {
                        dt { "Time" }
                        dd {
                            time_ago { timestamp_ns: ptx.timestamp.clone() }
                        }
                    }
                    // Gas
                    div {
                        dt { "Gas Used" }
                        dd {
                            gas_amount { gas: ptx.gas_burnt }
                        }
                    }
                }
            }

            // Explanation widgets (e.g., NEAR transfer, FT transfer)
            for widget in explanation_widgets {
                {(widget.render)(&tx_detail)}
            }

            // Actions
            if !ptx.actions.is_empty() {
                div { class: "detail-card",
                    div { class: "border-b border-gray-100 px-4 py-2",
                        h2 { class: "text-xs font-medium uppercase text-gray-500",
                            "Actions"
                        }
                    }
                    div { class: "px-4 py-3",
                        for (i , action) in ptx.actions.iter().enumerate() {
                            div { key: "{i}", class: "action-item",
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
                div { class: "detail-card",
                    div { class: "border-b border-gray-100 px-4 py-2",
                        h2 { class: "text-xs font-medium uppercase text-gray-500",
                            "Transfers"
                        }
                    }
                    div { class: "transfer-list",
                        for transfer in ptx.transfers {
                            div {
                                key: "ft-{transfer.amount}",
                                class: "transfer-item",
                                if let Some(from) = &transfer.from {
                                    span { "{from} → " }
                                }
                                if let Some(to) = &transfer.to {
                                    span { "{to}: " }
                                }
                                span { class: "font-mono", "{transfer.amount}" }
                                span { class: "text-gray-500",
                                    match transfer.token_type {
                                        crate::ui_utils::parse_transaction::TokenType::Near => " NEAR",
                                        crate::ui_utils::parse_transaction::TokenType::Nep141 => " (FT)",
                                        crate::ui_utils::parse_transaction::TokenType::Nep245 => " (MT)",
                                    }
                                }
                            }
                        }
                        for nft_transfer in ptx.nft_transfers {
                            div {
                                key: "nft-{nft_transfer.token_id}",
                                class: "transfer-item",
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
                    h2 { class: "section-heading", "Receipts ({ptx.receipts.len()})" }
                    div { class: "space-y-3",
                        for receipt in ptx.receipts {
                            div {
                                key: "{receipt.receipt.receipt_id}",
                                class: "receipt-card",
                                div { class: "receipt-grid",
                                    div {
                                        span { class: "text-gray-500", "From: " }
                                        account_id {
                                            account_id: receipt.receipt.predecessor_id.clone(),
                                            network: network,
                                        }
                                    }
                                    div {
                                        span { class: "text-gray-500", "To: " }
                                        account_id {
                                            account_id: receipt.receipt.receiver_id.clone(),
                                            network: network,
                                        }
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

            // Raw JSON widget (uses hook, must be component)
            DefaultWidget { tx_json: serde_json::to_string_pretty(&tx_detail).unwrap_or_default() }
        }
    }
}
// =========================================
// copyright 2026 by sleet.near

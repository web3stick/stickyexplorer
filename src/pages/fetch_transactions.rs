// pages/fetch_transactions.rs
// =========================================
// Shared transaction fetching with caching
// =========================================
use crate::api::types::TransactionDetail;
use crate::logic::tx_cache::TxCache;
use crate::utils::parse_transaction::{parse_transaction, ParsedTx};
use dioxus::prelude::*;
use reqwest::Client;
use serde::Serialize;
// =========================================

#[derive(Clone, Serialize)]
pub struct TxParams {
    pub tx_hashes: Vec<String>,
}

/// Fetch and parse transactions, using cache for known hashes
pub async fn fetch_and_parse_transactions(
    api_base: &str,
    hashes: &[String],
    tx_cache: &mut Signal<TxCache>,
) -> Vec<(String, ParsedTx)> {
    if hashes.is_empty() {
        return Vec::new();
    }

    // Check cache first
    let mut all_parsed = Vec::new();
    let mut missing_hashes = Vec::new();

    for hash in hashes {
        if let Some(parsed) = tx_cache.read().get(hash) {
            all_parsed.push((hash.clone(), parsed.clone()));
        } else {
            missing_hashes.push(hash.clone());
        }
    }

    if missing_hashes.is_empty() {
        return all_parsed;
    }

    // Fetch missing in batches
    const BATCH_SIZE: usize = 20;

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
                                serde_json::from_value::<TransactionDetail>(v.clone()).ok()
                            })
                            .map(|tx| {
                                let parsed = parse_transaction(&tx);
                                (parsed.hash.clone(), parsed.clone())
                            })
                            .collect();

                        tx_cache.write().insert_batch(parsed.clone());
                        all_parsed.extend(parsed);
                    }
                }
            }
            Err(_) => {}
        }

        gloo_timers::future::TimeoutFuture::new(50).await;
    }

    all_parsed
}
// =========================================
// copyright 2026 by sleet.near

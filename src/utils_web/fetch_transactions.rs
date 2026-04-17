// ui_utils/fetch_transactions.rs
// =========================================
// Shared transaction fetching with caching
// =========================================
use crate::api::client::ApiClient;
use crate::utils_web::tx_cache::TxCache;
use crate::utils::parse_transaction::{parse_transaction, ParsedTx};
use dioxus::prelude::*;
// =========================================

/// Fetch and parse transactions, using cache for known hashes
pub async fn fetch_and_parse_transactions(
    api_client: &ApiClient,
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
        let api_client = api_client.clone();
        let hashes_for_fetch = chunk.to_vec();

        match api_client.get_transactions(hashes_for_fetch.clone()).await {
            Ok(data) => {
                let parsed: Vec<(String, ParsedTx)> = data
                    .transactions
                    .into_iter()
                    .map(|tx| {
                        let parsed = parse_transaction(&tx);
                        (parsed.hash.clone(), parsed)
                    })
                    .collect();

                tx_cache.write().insert_batch(parsed.clone());
                all_parsed.extend(parsed);
            }
            Err(_) => {
                // Silently skip failed batches to match existing behavior
            }
        }

        gloo_timers::future::TimeoutFuture::new(50).await;
    }

    all_parsed
}
// =========================================
// copyright 2026 by sleet.near

// logic/tx_cache.rs
// =========================================
// Transaction cache for faster navigation
// =========================================
use crate::ui_utils::parse_transaction::ParsedTx;
use std::collections::HashMap;
// =========================================

/// Global transaction cache
pub struct TxCache {
    cache: HashMap<String, ParsedTx>,
}

impl TxCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn get(&self, hash: &str) -> Option<&ParsedTx> {
        self.cache.get(hash)
    }

    pub fn insert(&mut self, hash: String, tx: ParsedTx) {
        self.cache.insert(hash, tx);
    }

    pub fn insert_batch(&mut self, txs: Vec<(String, ParsedTx)>) {
        for (hash, tx) in txs {
            self.cache.insert(hash, tx);
        }
    }

    pub fn get_missing(&self, hashes: &[String]) -> Vec<String> {
        hashes
            .iter()
            .filter(|h| !self.cache.contains_key(*h))
            .cloned()
            .collect()
    }
}

impl Default for TxCache {
    fn default() -> Self {
        Self::new()
    }
}
// =========================================
// copyright 2026 by sleet.near

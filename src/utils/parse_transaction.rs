// utils/parse_transaction.rs
// =========================================
// Entry point for transaction parsing
// =========================================
use crate::api::types::TransactionDetail;
use crate::utils::extract_transfers::{extract_nft_transfers, extract_transfers, resolve_success};
use crate::utils::parse_action::parse_action;
pub use crate::utils::extract_transfers::{NftTransferInfo, TokenType, TransferInfo};
pub use crate::utils::parse_action::ParsedAction;
use serde::{Deserialize, Serialize};
// =========================================

/// Fully parsed transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedTx {
    pub hash: String,
    pub signer_id: String,
    pub receiver_id: String,
    pub block_height: u64,
    pub timestamp: String,
    pub gas_burnt: u64,
    pub is_success: Option<bool>,
    pub actions: Vec<ParsedAction>,
    pub relayer_id: Option<String>,
    pub transfers: Vec<TransferInfo>,
    pub nft_transfers: Vec<NftTransferInfo>,
    pub receipts: Vec<crate::api::types::ReceiptWithOutcome>,
}

/// Parse a transaction detail into a ParsedTx
pub fn parse_transaction(tx: &TransactionDetail) -> ParsedTx {
    let is_success = resolve_success(tx);

    let mut total_gas = tx.execution_outcome.outcome.gas_burnt;
    for r in &tx.receipts {
        total_gas += r.execution_outcome.outcome.gas_burnt;
    }

    let actions = tx.transaction.actions.clone();
    let mut parsed = ParsedTx {
        hash: tx.transaction.hash.clone(),
        signer_id: tx.transaction.signer_id.clone(),
        receiver_id: tx.transaction.receiver_id.clone(),
        block_height: tx.execution_outcome.block_height,
        timestamp: tx.execution_outcome.block_timestamp.to_string(),
        gas_burnt: total_gas,
        is_success,
        actions: actions.iter().map(parse_action).collect(),
        relayer_id: None,
        transfers: extract_transfers(tx),
        nft_transfers: extract_nft_transfers(tx),
        receipts: tx.receipts.clone(),
    };

    // Detect Delegate: first action is Delegate wrapping the real signer/receiver
    if actions.len() == 1 {
        if let crate::api::types::TransactionAction::Complex(v) = &actions[0] {
            if let Some(obj) = v.as_object() {
                if let Some(delegate) = obj.get("Delegate").and_then(|v| v.as_object()) {
                    if let Some(sender_id) = delegate.get("sender_id").and_then(|v| v.as_str()) {
                        parsed.relayer_id = Some(tx.transaction.signer_id.clone());
                        parsed.signer_id = sender_id.to_string();
                        if let Some(receiver_id) =
                            delegate.get("receiver_id").and_then(|v| v.as_str())
                        {
                            parsed.receiver_id = receiver_id.to_string();
                        }
                    }
                }
            }
        }
    }

    parsed
}
// =========================================
// copyright 2026 by sleet.near

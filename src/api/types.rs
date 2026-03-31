// api/types.rs
// =========================================
// API type definitions for NEAR Explorer
// =========================================
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// =========================================

/// /v0/blocks response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlocksResponse {
    pub blocks: Vec<BlockHeader>,
}

/// Block header information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub block_height: u64,
    pub block_hash: String,
    pub prev_block_hash: String,
    pub prev_block_height: Option<u64>,
    pub block_timestamp: String,
    pub block_ordinal: Option<u64>,
    pub gas_price: String,
    pub gas_burnt: String,
    pub total_supply: String,
    pub author_id: String,
    pub num_transactions: u32,
    pub num_receipts: u32,
    pub chunks_included: u32,
    pub epoch_id: String,
    pub next_epoch_id: String,
    pub protocol_version: u64,
    pub tokens_burnt: String,
}

/// /v0/block response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockTx {
    pub transaction_hash: String,
    pub signer_id: String,
    pub receiver_id: String,
    pub real_signer_id: String,
    pub real_receiver_id: String,
    pub tx_block_height: u64,
    pub tx_block_timestamp: String,
    pub tx_index: u32,
    pub gas_burnt: u64,
    pub is_success: bool,
    pub is_completed: bool,
    pub is_relayed: bool,
    pub tokens_burnt: String,
    pub shard_id: u32,
    pub nonce: u64,
    pub priority_fee: u64,
    pub signer_public_key: String,
    pub tx_block_hash: String,
    pub last_block_height: u64,
}

/// Block detail response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockDetailResponse {
    pub block: BlockHeader,
    pub block_txs: Vec<BlockTx>,
}

/// Transaction detail (deeply nested RPC-like structure)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionDetail {
    pub transaction: TransactionInfo,
    pub execution_outcome: ExecutionOutcomeInfo,
    pub receipts: Vec<ReceiptWithOutcome>,
    pub data_receipts: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInfo {
    pub signer_id: String,
    pub receiver_id: String,
    pub hash: String,
    pub actions: Vec<TransactionAction>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionOutcomeInfo {
    pub block_hash: String,
    pub block_height: u64,
    pub block_timestamp: u64,
    pub id: String,
    pub outcome: OutcomeData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutcomeData {
    pub executor_id: String,
    pub gas_burnt: u64,
    pub logs: Vec<String>,
    pub receipt_ids: Vec<String>,
    pub status: serde_json::Value,
    pub tokens_burnt: String,
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptWithOutcome {
    pub receipt: ReceiptInfo,
    pub execution_outcome: ExecutionOutcomeInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptInfo {
    pub block_hash: String,
    pub block_height: u64,
    pub block_timestamp: u64,
    pub predecessor_id: String,
    pub receiver_id: String,
    pub receipt_id: String,
    pub receipt: serde_json::Value,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Transaction action - can be string or object
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransactionAction {
    Simple(String),
    Complex(serde_json::Value),
}

/// /v0/transactions response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsResponse {
    pub transactions: Vec<TransactionDetail>,
}

/// /v0/account response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountResponse {
    pub account_txs: Vec<AccountTx>,
    pub resume_token: Option<String>,
    pub txs_count: u64,
}

/// Account transaction reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountTx {
    pub account_id: String,
    pub transaction_hash: String,
    pub tx_block_height: u64,
    pub tx_block_timestamp: String,
    pub tx_index: u32,
    pub is_success: bool,
    pub is_signer: bool,
    pub is_receiver: bool,
    pub is_real_signer: bool,
    pub is_real_receiver: bool,
    pub is_predecessor: bool,
    pub is_function_call: bool,
    pub is_any_signer: bool,
    pub is_delegated_signer: bool,
    pub is_event_log: bool,
    pub is_action_arg: bool,
    pub is_explicit_refund_to: bool,
}

/// Account filters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccountFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_signer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_delegated_signer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_real_signer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_any_signer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_predecessor: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_explicit_refund_to: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_receiver: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_real_receiver: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_function_call: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_action_arg: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_event_log: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_tx_block_height: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_tx_block_height: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<bool>,
}

/// Block filters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BlockFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_block_height: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_block_height: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<bool>,
}
// =========================================
// copyright 2026 by sleet.near

// utils/parse_transaction.rs
// =========================================
// Transaction parsing utilities
// =========================================
use crate::api::types::*;
use crate::utils::format::encode_base58;
use serde::{Deserialize, Serialize};
// =========================================

/// Parsed action from a transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedAction {
    pub action_type: String,
    pub method_name: Option<String>,
    pub deposit: Option<String>,
    pub args: Option<String>,
    pub gas: Option<u64>,
    pub public_key: Option<String>,
    pub access_key_permission: Option<String>,
    pub beneficiary_id: Option<String>,
    pub code_hash: Option<String>,
}

/// Transfer information (FT, MT, or NEAR)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferInfo {
    pub from: Option<String>,
    pub to: Option<String>,
    pub amount: String,
    pub token_type: TokenType,
    pub contract_id: Option<String>,
    pub token_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TokenType {
    Near,
    Nep141,
    Nep245,
}

/// NFT transfer information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftTransferInfo {
    pub from: Option<String>,
    pub to: Option<String>,
    pub contract_id: String,
    pub token_id: String,
}

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
    pub receipts: Vec<ReceiptWithOutcome>,
}

/// Parse a transaction action
pub fn parse_action(action: &TransactionAction) -> ParsedAction {
    match action {
        TransactionAction::Simple(s) => ParsedAction {
            action_type: s.clone(),
            method_name: None,
            deposit: None,
            args: None,
            gas: None,
            public_key: None,
            access_key_permission: None,
            beneficiary_id: None,
            code_hash: None,
        },
        TransactionAction::Complex(v) => {
            let mut result = ParsedAction {
                action_type: String::new(),
                method_name: None,
                deposit: None,
                args: None,
                gas: None,
                public_key: None,
                access_key_permission: None,
                beneficiary_id: None,
                code_hash: None,
            };

            if let Some(obj) = v.as_object() {
                // Get action type from first key
                if let Some(first_key) = obj.keys().next() {
                    result.action_type = first_key.clone();
                    if let Some(inner) = obj.get(first_key).and_then(|v| v.as_object()) {
                        if let Some(method_name) =
                            inner.get("method_name").and_then(|v| v.as_str())
                        {
                            result.method_name = Some(method_name.to_string());
                        }
                        if let Some(deposit) = inner.get("deposit").and_then(|v| v.as_str()) {
                            result.deposit = Some(deposit.to_string());
                        }
                        if let Some(args) = inner.get("args").and_then(|v| v.as_str()) {
                            result.args = Some(args.to_string());
                        }
                        if let Some(gas) = inner.get("gas").and_then(|v| v.as_u64()) {
                            result.gas = Some(gas);
                        }
                        if let Some(public_key) = inner.get("public_key").and_then(|v| v.as_str()) {
                            result.public_key = Some(public_key.to_string());
                        }
                        if let Some(beneficiary_id) =
                            inner.get("beneficiary_id").and_then(|v| v.as_str())
                        {
                            result.beneficiary_id = Some(beneficiary_id.to_string());
                        }
                        if let Some(code) = inner.get("code").and_then(|v| v.as_str()) {
                            if let Ok(bytes) = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, code) {
                                result.code_hash = Some(encode_base58(&bytes));
                            }
                        }
                        // Parse access key permission
                        if let Some(access_key) = inner.get("access_key").and_then(|v| v.as_object())
                        {
                            if let Some(permission) = access_key.get("permission") {
                                result.access_key_permission = Some(permission.to_string());
                            }
                        }
                    }
                }
                // Also check for flat structure (delegate actions)
                if result.action_type.is_empty() {
                    if let Some(action_type) = obj.get("type").and_then(|v| v.as_str()) {
                        result.action_type = action_type.to_string();
                    }
                    if let Some(method_name) = obj.get("method_name").and_then(|v| v.as_str()) {
                        result.method_name = Some(method_name.to_string());
                    }
                    if let Some(deposit) = obj.get("deposit").and_then(|v| v.as_str()) {
                        result.deposit = Some(deposit.to_string());
                    }
                }
            }

            result
        }
    }
}

/// Get deposit from a Transfer action
fn get_transfer_deposit(action: &TransactionAction) -> Option<String> {
    match action {
        TransactionAction::Simple(_) => None,
        TransactionAction::Complex(v) => {
            if let Some(obj) = v.as_object() {
                // Top-level format: { Transfer: { deposit: "..." } }
                if let Some(transfer) = obj.get("Transfer").and_then(|v| v.as_object()) {
                    return transfer.get("deposit").and_then(|v| v.as_str()).map(String::from);
                }
                // Delegate inner format: { type: "Transfer", deposit: "..." }
                if obj.get("type").and_then(|v| v.as_str()) == Some("Transfer") {
                    return obj.get("deposit").and_then(|v| v.as_str()).map(String::from);
                }
            }
            None
        }
    }
}

/// Get beneficiary from DeleteAccount action
fn get_delete_account_beneficiary(action: &TransactionAction) -> Option<String> {
    match action {
        TransactionAction::Simple(_) => None,
        TransactionAction::Complex(v) => {
            if let Some(obj) = v.as_object() {
                if let Some(delete) = obj.get("DeleteAccount").and_then(|v| v.as_object()) {
                    return delete
                        .get("beneficiary_id")
                        .and_then(|v| v.as_str())
                        .map(String::from);
                }
                if obj.get("type").and_then(|v| v.as_str()) == Some("DeleteAccount") {
                    return obj
                        .get("beneficiary_id")
                        .and_then(|v| v.as_str())
                        .map(String::from);
                }
            }
            None
        }
    }
}

/// Extract transfers from transaction
fn extract_transfers(tx: &TransactionDetail) -> Vec<TransferInfo> {
    let mut transfers = Vec::new();

    // Determine real signer/receiver/actions (handling delegates)
    let mut signer = tx.transaction.signer_id.clone();
    let mut receiver = tx.transaction.receiver_id.clone();
    let mut actions = tx.transaction.actions.clone();

    if actions.len() == 1 {
        if let TransactionAction::Complex(v) = &actions[0] {
            if let Some(obj) = v.as_object() {
                if let Some(delegate) = obj.get("Delegate").and_then(|v| v.as_object()) {
                    if let Some(sender_id) =
                        delegate.get("sender_id").and_then(|v| v.as_str())
                    {
                        signer = sender_id.to_string();
                    }
                    if let Some(receiver_id) =
                        delegate.get("receiver_id").and_then(|v| v.as_str())
                    {
                        receiver = receiver_id.to_string();
                    }
                    if let Some(inner_actions) =
                        delegate.get("actions").and_then(|v| v.as_array())
                    {
                        actions = inner_actions
                            .iter()
                            .map(|a| TransactionAction::Complex(a.clone()))
                            .collect();
                    }
                }
            }
        }
    }

    // 1. Native NEAR transfers from actions
    for action in &actions {
        if let Some(deposit) = get_transfer_deposit(action) {
            if deposit != "0" {
                transfers.push(TransferInfo {
                    from: Some(signer.clone()),
                    to: Some(receiver.clone()),
                    amount: deposit,
                    token_type: TokenType::Near,
                    contract_id: None,
                    token_id: None,
                });
            }
        }
    }

    // 2. DeleteAccount — remaining balance goes to beneficiary
    for action in &actions {
        if let Some(beneficiary) = get_delete_account_beneficiary(action) {
            for r in &tx.receipts {
                if r.receipt.predecessor_id == "system"
                    && r.receipt.receiver_id == beneficiary
                {
                    if let Some(action_data) = r
                        .receipt
                        .receipt
                        .as_object()
                        .and_then(|o| o.get("Action"))
                        .and_then(|v| v.as_object())
                    {
                        if let Some(receipt_actions) = action_data
                            .get("actions")
                            .and_then(|v| v.as_array())
                        {
                            for ra in receipt_actions {
                                let ra_action = TransactionAction::Complex(ra.clone());
                                if let Some(deposit) = get_transfer_deposit(&ra_action) {
                                    if deposit != "0" {
                                        transfers.push(TransferInfo {
                                            from: Some(receiver.clone()),
                                            to: Some(beneficiary.clone()),
                                            amount: deposit,
                                            token_type: TokenType::Near,
                                            contract_id: None,
                                            token_id: None,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // 3. Event logs from all receipts (NEP-141 and NEP-245)
    for r in &tx.receipts {
        let receipt_contract_id = r.receipt.receiver_id.clone();
        for log in &r.execution_outcome.outcome.logs {
            if !log.starts_with("EVENT_JSON:") {
                continue;
            }
            if let Ok(evt) = serde_json::from_str::<serde_json::Value>(&log[11..]) {
                if let Some(standard) = evt.get("standard").and_then(|v| v.as_str()) {
                    if let Some(data) = evt.get("data").and_then(|v| v.as_array()) {
                        // NEP-141 single-token events
                        if standard == "nep141" {
                            for item in data {
                                let amount = item
                                    .get("amount")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("0");
                                if amount == "0" {
                                    continue;
                                }
                                if let Some(event) = evt.get("event").and_then(|v| v.as_str()) {
                                    let (from, to) = match event {
                                        "ft_transfer" => (
                                            item.get("old_owner_id").and_then(|v| v.as_str()).map(String::from),
                                            item.get("new_owner_id").and_then(|v| v.as_str()).map(String::from),
                                        ),
                                        "ft_mint" => (
                                            None,
                                            item.get("owner_id").and_then(|v| v.as_str()).map(String::from),
                                        ),
                                        "ft_burn" => (
                                            item.get("owner_id").and_then(|v| v.as_str()).map(String::from),
                                            None,
                                        ),
                                        _ => continue,
                                    };
                                    transfers.push(TransferInfo {
                                        from,
                                        to,
                                        amount: amount.to_string(),
                                        token_type: TokenType::Nep141,
                                        contract_id: Some(receipt_contract_id.clone()),
                                        token_id: None,
                                    });
                                }
                            }
                        }
                        // NEP-245 multi-token events
                        if standard == "nep245" {
                            for item in data {
                                let token_ids = item
                                    .get("token_ids")
                                    .and_then(|v| v.as_array())
                                    .cloned()
                                    .unwrap_or_default();
                                let amounts = item
                                    .get("amounts")
                                    .and_then(|v| v.as_array())
                                    .cloned()
                                    .unwrap_or_default();
                                for (i, tid) in token_ids.iter().enumerate() {
                                    let amount = amounts
                                        .get(i)
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("0");
                                    if amount == "0" {
                                        continue;
                                    }
                                    if let Some(event) = evt.get("event").and_then(|v| v.as_str()) {
                                        let (from, to) = match event {
                                            "mt_transfer" => (
                                                item.get("old_owner_id").and_then(|v| v.as_str()).map(String::from),
                                                item.get("new_owner_id").and_then(|v| v.as_str()).map(String::from),
                                            ),
                                            "mt_mint" => (
                                                None,
                                                item.get("owner_id").and_then(|v| v.as_str()).map(String::from),
                                            ),
                                            "mt_burn" => (
                                                item.get("owner_id").and_then(|v| v.as_str()).map(String::from),
                                                None,
                                            ),
                                            _ => continue,
                                        };
                                        transfers.push(TransferInfo {
                                            from,
                                            to,
                                            amount: amount.to_string(),
                                            token_type: TokenType::Nep245,
                                            contract_id: Some(receipt_contract_id.clone()),
                                            token_id: tid.as_str().map(String::from),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    transfers
}

/// Extract NFT transfers from transaction
fn extract_nft_transfers(tx: &TransactionDetail) -> Vec<NftTransferInfo> {
    let mut nft_transfers = Vec::new();

    for r in &tx.receipts {
        let receipt_contract_id = r.receipt.receiver_id.clone();
        for log in &r.execution_outcome.outcome.logs {
            if !log.starts_with("EVENT_JSON:") {
                continue;
            }
            if let Ok(evt) = serde_json::from_str::<serde_json::Value>(&log[11..]) {
                if let Some(standard) = evt.get("standard").and_then(|v| v.as_str()) {
                    if standard != "nep171" {
                        continue;
                    }
                    if let Some(data) = evt.get("data").and_then(|v| v.as_array()) {
                        for item in data {
                            if let Some(token_ids) =
                                item.get("token_ids").and_then(|v| v.as_array())
                            {
                                for tid in token_ids {
                                    if let Some(token_id) = tid.as_str() {
                                        if let Some(event) =
                                            evt.get("event").and_then(|v| v.as_str())
                                        {
                                            let (from, to) = match event {
                                                "nft_transfer" => (
                                                    item.get("old_owner_id").and_then(|v| v.as_str()).map(String::from),
                                                    item.get("new_owner_id").and_then(|v| v.as_str()).map(String::from),
                                                ),
                                                "nft_mint" => (
                                                    None,
                                                    item.get("owner_id").and_then(|v| v.as_str()).map(String::from),
                                                ),
                                                "nft_burn" => (
                                                    item.get("owner_id").and_then(|v| v.as_str()).map(String::from),
                                                    None,
                                                ),
                                                _ => continue,
                                            };
                                            nft_transfers.push(NftTransferInfo {
                                                from,
                                                to,
                                                contract_id: receipt_contract_id.clone(),
                                                token_id: token_id.to_string(),
                                            });
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

    nft_transfers
}

/// Resolve transaction success status
fn resolve_success(tx: &TransactionDetail) -> Option<bool> {
    let status = &tx.execution_outcome.outcome.status;
    if status.get("Failure").is_some() {
        return Some(false);
    }
    if status.get("SuccessValue").is_some() {
        return Some(true);
    }
    if let Some(receipt_id) = status.get("SuccessReceiptId").and_then(|v| v.as_str()) {
        if let Some(receipt) = tx.receipts.iter().find(|r| r.receipt.receipt_id == receipt_id) {
            let r_status = &receipt.execution_outcome.outcome.status;
            if r_status.get("Failure").is_some() {
                return Some(false);
            }
            if r_status.get("SuccessValue").is_some() {
                return Some(true);
            }
            if r_status.get("SuccessReceiptId").is_some() {
                return Some(true);
            }
        }
    }
    None
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
        if let TransactionAction::Complex(v) = &actions[0] {
            if let Some(obj) = v.as_object() {
                if let Some(delegate) = obj.get("Delegate").and_then(|v| v.as_object()) {
                    if let Some(sender_id) =
                        delegate.get("sender_id").and_then(|v| v.as_str())
                    {
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

// utils/extract_transfers.rs
// =========================================
// Extract transfers and NFTs from transaction receipts
// =========================================
use crate::api::types::{TransactionAction, TransactionDetail};
use crate::utils::parse_action::{get_delete_account_beneficiary, get_transfer_deposit};
// =========================================

/// Transfer information (FT, MT, or NEAR)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TransferInfo {
    pub from: Option<String>,
    pub to: Option<String>,
    pub amount: String,
    pub token_type: TokenType,
    pub contract_id: Option<String>,
    pub token_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TokenType {
    Near,
    Nep141,
    Nep245,
}

/// NFT transfer information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NftTransferInfo {
    pub from: Option<String>,
    pub to: Option<String>,
    pub contract_id: String,
    pub token_id: String,
}

/// Extract transfers from transaction
pub fn extract_transfers(tx: &TransactionDetail) -> Vec<TransferInfo> {
    let mut transfers = Vec::new();

    // Determine real signer/receiver/actions (handling delegates)
    let mut signer = tx.transaction.signer_id.clone();
    let mut receiver = tx.transaction.receiver_id.clone();
    let mut actions = tx.transaction.actions.clone();

    if actions.len() == 1 {
        if let TransactionAction::Complex(v) = &actions[0] {
            if let Some(obj) = v.as_object() {
                if let Some(delegate) = obj.get("Delegate").and_then(|v| v.as_object()) {
                    if let Some(sender_id) = delegate.get("sender_id").and_then(|v| v.as_str()) {
                        signer = sender_id.to_string();
                    }
                    if let Some(receiver_id) = delegate.get("receiver_id").and_then(|v| v.as_str())
                    {
                        receiver = receiver_id.to_string();
                    }
                    if let Some(inner_actions) = delegate.get("actions").and_then(|v| v.as_array())
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
                if r.receipt.predecessor_id == "system" && r.receipt.receiver_id == beneficiary {
                    if let Some(action_data) = r
                        .receipt
                        .receipt
                        .as_object()
                        .and_then(|o| o.get("Action"))
                        .and_then(|v| v.as_object())
                    {
                        if let Some(receipt_actions) =
                            action_data.get("actions").and_then(|v| v.as_array())
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
                                let amount =
                                    item.get("amount").and_then(|v| v.as_str()).unwrap_or("0");
                                if amount == "0" {
                                    continue;
                                }
                                if let Some(event) = evt.get("event").and_then(|v| v.as_str()) {
                                    let (from, to) = match event {
                                        "ft_transfer" => (
                                            item.get("old_owner_id")
                                                .and_then(|v| v.as_str())
                                                .map(String::from),
                                            item.get("new_owner_id")
                                                .and_then(|v| v.as_str())
                                                .map(String::from),
                                        ),
                                        "ft_mint" => (
                                            None,
                                            item.get("owner_id")
                                                .and_then(|v| v.as_str())
                                                .map(String::from),
                                        ),
                                        "ft_burn" => (
                                            item.get("owner_id")
                                                .and_then(|v| v.as_str())
                                                .map(String::from),
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
                                    let amount =
                                        amounts.get(i).and_then(|v| v.as_str()).unwrap_or("0");
                                    if amount == "0" {
                                        continue;
                                    }
                                    if let Some(event) = evt.get("event").and_then(|v| v.as_str()) {
                                        let (from, to) = match event {
                                            "mt_transfer" => (
                                                item.get("old_owner_id")
                                                    .and_then(|v| v.as_str())
                                                    .map(String::from),
                                                item.get("new_owner_id")
                                                    .and_then(|v| v.as_str())
                                                    .map(String::from),
                                            ),
                                            "mt_mint" => (
                                                None,
                                                item.get("owner_id")
                                                    .and_then(|v| v.as_str())
                                                    .map(String::from),
                                            ),
                                            "mt_burn" => (
                                                item.get("owner_id")
                                                    .and_then(|v| v.as_str())
                                                    .map(String::from),
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
pub fn extract_nft_transfers(tx: &TransactionDetail) -> Vec<NftTransferInfo> {
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
                                                    item.get("old_owner_id")
                                                        .and_then(|v| v.as_str())
                                                        .map(String::from),
                                                    item.get("new_owner_id")
                                                        .and_then(|v| v.as_str())
                                                        .map(String::from),
                                                ),
                                                "nft_mint" => (
                                                    None,
                                                    item.get("owner_id")
                                                        .and_then(|v| v.as_str())
                                                        .map(String::from),
                                                ),
                                                "nft_burn" => (
                                                    item.get("owner_id")
                                                        .and_then(|v| v.as_str())
                                                        .map(String::from),
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
pub fn resolve_success(tx: &TransactionDetail) -> Option<bool> {
    let status = &tx.execution_outcome.outcome.status;
    if status.get("Failure").is_some() {
        return Some(false);
    }
    if status.get("SuccessValue").is_some() {
        return Some(true);
    }
    if let Some(receipt_id) = status.get("SuccessReceiptId").and_then(|v| v.as_str()) {
        if let Some(receipt) = tx
            .receipts
            .iter()
            .find(|r| r.receipt.receipt_id == receipt_id)
        {
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
// =========================================
// copyright 2026 by sleet.near

// utils/parse_action.rs
// =========================================
// Parse transaction actions into structured form
// =========================================
use crate::api::types::TransactionAction;
use crate::ui_utils::format::encode_base58;
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

/// Parse a transaction action into a ParsedAction
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
                        if let Some(method_name) = inner.get("method_name").and_then(|v| v.as_str())
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
                            if let Ok(bytes) = base64::Engine::decode(
                                &base64::engine::general_purpose::STANDARD,
                                code,
                            ) {
                                result.code_hash = Some(encode_base58(&bytes));
                            }
                        }
                        // Parse access key permission
                        if let Some(access_key) =
                            inner.get("access_key").and_then(|v| v.as_object())
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
pub fn get_transfer_deposit(action: &TransactionAction) -> Option<String> {
    match action {
        TransactionAction::Simple(_) => None,
        TransactionAction::Complex(v) => {
            if let Some(obj) = v.as_object() {
                // Top-level format: { Transfer: { deposit: "..." } }
                if let Some(transfer) = obj.get("Transfer").and_then(|v| v.as_object()) {
                    return transfer
                        .get("deposit")
                        .and_then(|v| v.as_str())
                        .map(String::from);
                }
                // Delegate inner format: { type: "Transfer", deposit: "..." }
                if obj.get("type").and_then(|v| v.as_str()) == Some("Transfer") {
                    return obj
                        .get("deposit")
                        .and_then(|v| v.as_str())
                        .map(String::from);
                }
            }
            None
        }
    }
}

/// Get beneficiary from DeleteAccount action
pub fn get_delete_account_beneficiary(action: &TransactionAction) -> Option<String> {
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
// =========================================
// copyright 2026 by sleet.near

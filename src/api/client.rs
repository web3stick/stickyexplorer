// api/client.rs
// =========================================
// API client for NEAR Explorer with logging
// =========================================
use crate::api::types::*;
use reqwest::Client;
use serde::Serialize;
// =========================================

/// API client for FastNear
#[derive(Clone)]
pub struct ApiClient {
    client: Client,
    base_url: String,
    network: String,
}

impl ApiClient {
    /// Create a new API client for a given network
    pub fn new(base_url: impl Into<String>, network: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into(),
            network: network.into(),
        }
    }

    /// Log request to console as JSON object (devtools shows collapsible tree)
    fn log_request(&self, endpoint: &str, params: &impl serde::Serialize) {
        let url = format!("{}/v0/{}", self.base_url, endpoint);
        let params_json = serde_json::to_value(params).unwrap_or_default();
        let log_obj = serde_json::json!({
            "type": "REQUEST",
            "network": self.network,
            "endpoint": endpoint,
            "url": url,
            "params": params_json,
        });
        if let Ok(js_val) = serde_wasm_bindgen::to_value(&log_obj) {
            web_sys::console::log_1(&js_val);
        }
    }

    /// Log response to console as JSON object (devtools shows collapsible tree)
    fn log_response(&self, endpoint: &str, status: u16, body: &str) {
        let body_preview = if body.len() > 500 { format!("{}...(truncated)", &body[..500]) } else { body.to_string() };
        let body_json: serde_json::Value = if body.len() <= 500 {
            serde_json::from_str(body).unwrap_or(serde_json::Value::String(body.to_string()))
        } else {
            serde_json::Value::String(body_preview.clone())
        };
        let log_obj = serde_json::json!({
            "type": "RESPONSE",
            "network": self.network,
            "endpoint": endpoint,
            "url": format!("{}/v0/{}", self.base_url, endpoint),
            "status": status,
            "body": body_json,
        });
        if let Ok(js_val) = serde_wasm_bindgen::to_value(&log_obj) {
            web_sys::console::log_1(&js_val);
        }
    }

    /// Fetch from API endpoint
    async fn fetch_api<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        body: impl Serialize,
    ) -> Result<T, String> {
        let url = format!("{}/v0/{}", self.base_url, endpoint);
        self.log_request(endpoint, &body);
        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        let status = response.status().as_u16();
        let body_text = response.text().await.unwrap_or_default();
        self.log_response(endpoint, status, &body_text);
        let parsed: T = match serde_json::from_str(&body_text) {
            Ok(v) => v,
            Err(e) => {
                let err_obj = serde_json::json!({
                    "type": "ERROR",
                    "network": self.network,
                    "endpoint": endpoint,
                    "error": e.to_string(),
                    "body_preview": &body_text[..body_text.len().min(200)],
                });
                if let Ok(js_val) = serde_wasm_bindgen::to_value(&err_obj) {
                    web_sys::console::log_1(&js_val);
                }
                return Err(e.to_string());
            }
        };
        Ok(parsed)
    }

    /// Get blocks
    pub async fn get_blocks(
        &self,
        limit: Option<u32>,
        desc: Option<bool>,
        to_block_height: Option<u64>,
        from_block_height: Option<u64>,
    ) -> Result<BlocksResponse, String> {
        #[derive(Serialize)]
        struct Params {
            #[serde(skip_serializing_if = "Option::is_none")]
            limit: Option<u32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            desc: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            to_block_height: Option<u64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            from_block_height: Option<u64>,
        }
        self.fetch_api(
            "blocks",
            Params {
                limit,
                desc,
                to_block_height,
                from_block_height,
            },
        )
        .await
    }

    /// Get block by ID (height or hash)
    pub async fn get_block(
        &self,
        block_id: BlockId,
        with_transactions: bool,
    ) -> Result<BlockDetailResponse, String> {
        // Use an untagged enum to serialize block_id correctly:
        // - Height variant serializes as a raw number (e.g., 100000000)
        // - Hash variant serializes as a string (e.g., "abc123")
        #[derive(Serialize)]
        #[serde(untagged)]
        enum BlockIdParam {
            Height(u64),
            Hash(String),
        }
        #[derive(Serialize)]
        struct Params {
            block_id: BlockIdParam,
            with_transactions: bool,
        }
        let params = match block_id {
            BlockId::Height(h) => Params {
                block_id: BlockIdParam::Height(h),
                with_transactions,
            },
            BlockId::Hash(h) => Params {
                block_id: BlockIdParam::Hash(h),
                with_transactions,
            },
        };
        self.fetch_api("block", params).await
    }

    /// Get transactions by hashes
    pub async fn get_transactions(
        &self,
        tx_hashes: Vec<String>,
    ) -> Result<TransactionsResponse, String> {
        #[derive(Serialize)]
        struct Params {
            tx_hashes: Vec<String>,
        }
        self.fetch_api("transactions", Params { tx_hashes }).await
    }

    /// Get account transactions
    pub async fn get_account(
        &self,
        account_id: &str,
        filters: &AccountFilters,
        resume_token: Option<&str>,
        limit: Option<u32>,
    ) -> Result<AccountResponse, String> {
        #[derive(Serialize)]
        struct Params<'a> {
            account_id: &'a str,
            #[serde(flatten)]
            filters: &'a AccountFilters,
            #[serde(skip_serializing_if = "Option::is_none")]
            resume_token: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            limit: Option<u32>,
        }
        self.fetch_api(
            "account",
            Params {
                account_id,
                filters,
                resume_token,
                limit,
            },
        )
        .await
    }
}

/// Block identifier - can be height or hash
pub enum BlockId {
    Height(u64),
    Hash(String),
}
// =========================================
// copyright 2026 by sleet.near

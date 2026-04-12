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

    /// Create client for mainnet
    pub fn mainnet() -> Self {
        Self::new("https://api.fastnear.com", "mainnet")
    }

    /// Create client for testnet
    pub fn testnet() -> Self {
        Self::new("https://api-testnet.fastnear.com", "testnet")
    }

    /// Log request to console
    fn log_request(&self, endpoint: &str, params: &impl serde::Serialize) {
        let url = format!("{}/v0/{}", self.base_url, endpoint);
        let params_json = serde_json::to_string(params).unwrap_or_default();
        web_sys::console::log_1(&"============".into());
        web_sys::console::log_1(&format!("[{}] REQUEST: {}", self.network, url).into());
        web_sys::console::log_1(&format!("params: {}", params_json).into());
        web_sys::console::log_1(&"============".into());
    }

    /// Log response to console
    fn log_response(&self, endpoint: &str, status: u16, body: &str) {
        web_sys::console::log_1(&"============".into());
        web_sys::console::log_1(&format!("[{}] RESPONSE: {}/v0/{}", self.network, self.base_url, endpoint).into());
        web_sys::console::log_1(&format!("status: {}", status).into());
        // Log first 500 chars of body
        let preview = if body.len() > 500 { format!("{}...(truncated)", &body[..500]) } else { body.to_string() };
        web_sys::console::log_1(&format!("body: {}", preview).into());
        web_sys::console::log_1(&"============".into());
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
                web_sys::console::log_1(&format!("JSON parse error: {}", e).into());
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
        #[derive(Serialize)]
        struct Params {
            block_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            with_transactions: Option<bool>,
        }
        let block_id_str = match block_id {
            BlockId::Height(h) => h.to_string(),
            BlockId::Hash(h) => h,
        };
        self.fetch_api(
            "block",
            Params {
                block_id: block_id_str,
                with_transactions: Some(with_transactions).filter(|v| *v),
            },
        )
        .await
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

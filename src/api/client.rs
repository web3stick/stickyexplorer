// api/client.rs
// =========================================
// API client for NEAR Explorer
// =========================================
use crate::api::types::*;
use reqwest::Client;
use serde::Serialize;
use web_sys::console;
// =========================================

fn log_to_console(msg: &str) {
    console::log_1(&msg.into());
}

fn get_network(base_url: &str) -> &str {
    if base_url.contains("testnet") {
        "testnet"
    } else {
        "mainnet"
    }
}

/// API client for FastNear
pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    /// Create a new API client
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into(),
        }
    }

    /// Fetch raw text from API endpoint
    async fn fetch_api_raw(
        &self,
        endpoint: &str,
        body: impl Serialize,
    ) -> Result<String, reqwest::Error> {
        let network = get_network(&self.base_url);
        let url = format!("{}/v0/{}", self.base_url, endpoint);
        let body_json = serde_json::to_string(&body).unwrap_or_default();

        // =========================================
        // LOG: API request with full URL and params
        // =========================================
        let request_log = format!(
            "============ API REQUEST ============\n\
            endpoint: {}\n\
            network: {}\n\
            url: {}\n\
            params: {}",
            endpoint, network, url, body_json
        );
        log_to_console(&request_log);

        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        let response_text = response.text().await?;

        // =========================================
        // LOG: API response with full data
        // =========================================
        let response_log = format!(
            "============ API RESPONSE ============\n\
            endpoint: {}\n\
            network: {}\n\
            response: {}",
            endpoint, network, response_text
        );
        log_to_console(&response_log);

        Ok(response_text)
    }

    /// Parse raw JSON response (panics on failure — raw response logged above)
    fn parse_response<T: serde::de::DeserializeOwned>(text: &str) -> T {
        serde_json::from_str(text).expect("failed to parse API response (logged above)")
    }

    /// Get blocks
    pub async fn get_blocks(
        &self,
        limit: Option<u32>,
        desc: Option<bool>,
        to_block_height: Option<u64>,
        from_block_height: Option<u64>,
    ) -> Result<BlocksResponse, reqwest::Error> {
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

        let raw = self
            .fetch_api_raw(
                "blocks",
                Params {
                    limit,
                    desc,
                    to_block_height,
                    from_block_height,
                },
            )
            .await?;
        Ok(Self::parse_response(&raw))
    }

    /// Get block by ID (height or hash)
    pub async fn get_block(
        &self,
        block_id: BlockId,
        with_transactions: bool,
    ) -> Result<BlockDetailResponse, reqwest::Error> {
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

        let raw = self
            .fetch_api_raw(
                "block",
                Params {
                    block_id: block_id_str,
                    with_transactions: Some(with_transactions).filter(|v| *v),
                },
            )
            .await?;
        Ok(Self::parse_response(&raw))
    }

    /// Get transactions by hashes
    pub async fn get_transactions(
        &self,
        tx_hashes: Vec<String>,
    ) -> Result<TransactionsResponse, reqwest::Error> {
        #[derive(Serialize)]
        struct Params {
            tx_hashes: Vec<String>,
        }

        let raw = self.fetch_api_raw("transactions", Params { tx_hashes }).await?;
        Ok(Self::parse_response(&raw))
    }

    /// Get account transactions
    pub async fn get_account(
        &self,
        account_id: &str,
        filters: &AccountFilters,
        resume_token: Option<&str>,
        limit: Option<u32>,
    ) -> Result<AccountResponse, reqwest::Error> {
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

        let raw = self
            .fetch_api_raw(
                "account",
                Params {
                    account_id,
                    filters,
                    resume_token,
                    limit,
                },
            )
            .await?;
        Ok(Self::parse_response(&raw))
    }
}

/// Block identifier - can be height or hash
pub enum BlockId {
    Height(u64),
    Hash(String),
}
// =========================================
// copyright 2026 by sleet.near

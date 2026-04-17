// iced_pages/app.rs
// =========================================
// Main Iced application state and orchestration
// =========================================
use crate::api::types::{
    BlockDetailResponse, BlockHeader, TransactionDetail,
};
use crate::iced_components::nav::NavbarState;
use crate::iced_components::search_bar::SearchBarState;
use crate::iced_pages::iced_app;
use crate::utils_iced::network::NetworkId;
use crate::utils::parse_transaction::ParsedTx;
use iced::{Element, Task};
use std::collections::HashMap;

// =========================================

/// Application-wide message type
#[derive(Debug, Clone)]
pub enum Message {
    // Navigation
    NavigateTo(Page),
    // Network
    SetNetwork(NetworkId),
    // Search
    SearchInput(String),
    SearchSubmit,
    // Home page
    LoadBlocks,
    BlocksLoaded(Vec<BlockHeader>),
    LoadBlocksFailed(String),
    LoadMoreBlocks,
    AppendBlocks(Vec<BlockHeader>, bool, Option<u64>),
    // Account page
    LoadAccount(String),
    AccountLoaded(Vec<crate::api::types::AccountTx>, HashMap<String, ParsedTx>, u64),
    LoadMoreAccount,
    AppendAccount(Vec<crate::api::types::AccountTx>, HashMap<String, ParsedTx>),
    // Block page
    LoadBlock(String),
    BlockLoaded(BlockDetailResponse),
    BlockLoadFailed(String),
    // Tx page
    LoadTx(String),
    TxLoaded(TransactionDetail),
    TxLoadFailed(String),
    // Tick (for time updates)
    Tick,
}

/// Page types
#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    Home,
    Account(String),
    Block(String),
    Tx(String),
}

impl std::fmt::Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Page::Home => write!(f, "home"),
            Page::Account(_) => write!(f, "account"),
            Page::Block(_) => write!(f, "block"),
            Page::Tx(_) => write!(f, "tx"),
        }
    }
}

impl Page {
    pub fn page_name(&self) -> &'static str {
        match self {
            Page::Home => "home",
            Page::Account(_) => "account",
            Page::Block(_) => "block",
            Page::Tx(_) => "tx",
        }
    }
}

/// Application state
#[derive(Clone)]
pub struct AppState {
    pub current_page: Page,
    pub network: NetworkId,
    pub search_query: String,

    // Home state
    pub blocks: Vec<BlockHeader>,
    pub blocks_loading: bool,
    pub blocks_error: Option<String>,
    pub blocks_resume_height: Option<u64>,
    pub blocks_has_more: bool,
    pub loading_more_blocks: bool,

    // Account state
    pub account_txs: Vec<crate::api::types::AccountTx>,
    pub account_parsed: HashMap<String, ParsedTx>,
    pub account_loading: bool,
    pub account_resume_token: Option<String>,
    pub account_has_more: bool,
    pub account_txs_count: u64,
    pub loading_more_account: bool,

    // Block state
    pub block_detail: Option<BlockDetailResponse>,
    pub block_loading: bool,
    pub block_error: Option<String>,

    // Tx state
    pub tx_detail: Option<TransactionDetail>,
    pub tx_parsed: Option<ParsedTx>,
    pub tx_loading: bool,
    pub tx_error: Option<String>,

    // UI state
    pub navbar: NavbarState,
    pub search: SearchBarState,
}

impl AppState {
    /// Create new app state and a startup Task that loads blocks.
    /// This signature matches what iced::application() expects for its boot function.
    pub fn new() -> (Self, Task<Message>) {
        let state = Self {
            current_page: Page::Home,
            network: NetworkId::Mainnet,
            search_query: String::new(),
            blocks: Vec::new(),
            blocks_loading: true,
            blocks_error: None,
            blocks_resume_height: None,
            blocks_has_more: true,
            loading_more_blocks: false,
            account_txs: Vec::new(),
            account_parsed: HashMap::new(),
            account_loading: false,
            account_resume_token: None,
            account_has_more: false,
            account_txs_count: 0,
            loading_more_account: false,
            block_detail: None,
            block_loading: false,
            block_error: None,
            tx_detail: None,
            tx_parsed: None,
            tx_loading: false,
            tx_error: None,
            navbar: NavbarState::new(),
            search: SearchBarState::new(),
        };
        // Startup task: load blocks from the API
        let startup_task = Task::future(async move {
            let api = crate::api::client::ApiClient::new(
                state.network.api_base_url(),
                state.network.as_str(),
            );
            match api.get_blocks(Some(80), Some(true), None, None).await {
                Ok(data) => Message::BlocksLoaded(data.blocks),
                Err(e) => Message::LoadBlocksFailed(e),
            }
        });
        (state, startup_task)
    }

    /// Mutable update — delegates to iced_app::update.
    /// This is called by the binary's update closure.
    pub fn update(&mut self, msg: Message) -> Task<Message> {
        iced_app::update(msg, self)
    }

    /// View — delegates to iced_app::view.
    pub fn view(&self) -> Element<'_, Message> {
        iced_app::view(self)
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new().0
    }
}

// =========================================
// Helper functions for styled text
// =========================================

/// Create a label-style text element (small, gray)
pub fn label_text(label: impl Into<String>) -> iced_widget::text::Text<'static> {
    iced_widget::text::Text::new(label.into())
        .size(12)
        .color(iced::Color::from_rgb(0.5, 0.5, 0.6))
}

/// Create a value-style text element (normal, light)
pub fn value_text(value: impl Into<String>) -> iced_widget::text::Text<'static> {
    iced_widget::text::Text::new(value.into())
        .size(13)
        .color(iced::Color::from_rgb(0.9, 0.9, 0.95))
}

/// Create a monospace text element (for hashes, addresses, etc.)
pub fn mono_text(value: impl Into<String>) -> iced_widget::text::Text<'static> {
    iced_widget::text::Text::new(value.into())
        .size(12)
        .color(iced::Color::from_rgb(0.7, 0.95, 0.7))
}

/// Create a heading-style text element
pub fn heading_text<'a>(heading: &'a str) -> iced_widget::text::Text<'a> {
    iced_widget::text::Text::new(heading)
        .size(18)
        .color(iced::Color::from_rgb(0.9, 0.9, 0.95))
}

// =========================================

// iced_pages/iced_app.rs
// =========================================
// Main Iced application - update and view
// =========================================
use crate::api::client::{ApiClient, BlockId};
use crate::api::types::AccountFilters;
use crate::iced_pages::app::{AppState, Message, Page};
use crate::iced_pages::home_page::HomePage;
use crate::iced_pages::account_page::AccountPage;
use crate::iced_pages::block_page::BlockPage;
use crate::iced_pages::tx_page::TxPage;
use crate::iced_components::button::network_toggle;
use crate::iced_components::SearchBar;
use crate::utils::network::NetworkId;
use crate::utils::parse_transaction::parse_transaction;
use iced::{Color, Element, Length, Task};
use iced::alignment::{Horizontal, Vertical};
use iced_widget::{button, column, container, row, Text, Space};
use std::collections::HashMap;

const APP_TITLE: &str = "StickyExplorer";

// =========================================

/// Detect search query type and route accordingly
fn detect_search_target(query: &str) -> (Page, NetworkId) {
    let stripped = query.replace(',', "");
    if stripped.chars().all(|c| c.is_ascii_digit()) {
        if query.ends_with(".testnet") {
            (Page::Block(query.replace(".testnet", "")), NetworkId::Testnet)
        } else {
            (Page::Block(stripped), NetworkId::Mainnet)
        }
    } else if query.len() < 50 {
        if query.ends_with(".testnet") {
            (Page::Account(query.to_string()), NetworkId::Testnet)
        } else {
            (Page::Account(query.to_string()), NetworkId::Mainnet)
        }
    } else {
        if query.ends_with(".testnet") {
            (Page::Tx(query.replace(".testnet", "")), NetworkId::Testnet)
        } else {
            (Page::Tx(query.to_string()), NetworkId::Mainnet)
        }
    }
}

/// Fetch and parse transaction details from API
async fn fetch_and_parse_tx_details(
    api: &ApiClient,
    hashes: &[String],
) -> Vec<(String, crate::utils::parse_transaction::ParsedTx)> {
    if hashes.is_empty() {
        return vec![];
    }
    match api.get_transactions(hashes.to_vec()).await {
        Ok(data) => data
            .transactions
            .into_iter()
            .map(|tx| {
                let parsed = parse_transaction(&tx);
                (parsed.hash.clone(), parsed)
            })
            .collect(),
        Err(_) => vec![],
    }
}

// =========================================
// UPDATE
// =========================================

pub fn update(msg: Message, state: &mut AppState) -> Task<Message> {
    match msg {
        Message::NavigateTo(page) => {
            state.current_page = page.clone();
            match &page {
                Page::Home => {
                    if state.blocks.is_empty() && state.blocks_loading {
                        return update(Message::LoadBlocks, state);
                    }
                }
                Page::Account(account_id) => {
                    state.account_txs.clear();
                    state.account_parsed.clear();
                    state.account_loading = true;
                    state.account_has_more = false;
                    let network = state.network;
                    let acc = account_id.clone();
                    return Task::future(async move {
                        let api = ApiClient::new(network.api_base_url(), network.as_str());
                        let filters = AccountFilters::default();
                        match api.get_account(&acc, &filters, None, Some(80)).await {
                            Ok(data) => {
                                let hashes: Vec<String> =
                                    data.account_txs.iter().map(|t| t.transaction_hash.clone()).collect();
                                let parsed = fetch_and_parse_tx_details(&api, &hashes).await;
                                let parsed_map: HashMap<String, _> = parsed.into_iter().collect();
                                Message::AccountLoaded(data.account_txs, parsed_map, data.txs_count)
                            }
                            Err(_) => Message::AccountLoaded(vec![], HashMap::new(), 0),
                        }
                    });
                }
                Page::Block(block_id) => {
                    state.block_detail = None;
                    state.block_loading = true;
                    state.block_error = None;
                    let network = state.network;
                    let bid = block_id.clone();
                    return Task::future(async move {
                        let api = ApiClient::new(network.api_base_url(), network.as_str());
                        let block_identifier = if let Ok(height) = bid.parse::<u64>() {
                            BlockId::Height(height)
                        } else {
                            BlockId::Hash(bid)
                        };
                        match api.get_block(block_identifier, true).await {
                            Ok(data) => Message::BlockLoaded(data),
                            Err(e) => Message::BlockLoadFailed(e),
                        }
                    });
                }
                Page::Tx(tx_hash) => {
                    state.tx_detail = None;
                    state.tx_parsed = None;
                    state.tx_loading = true;
                    state.tx_error = None;
                    let network = state.network;
                    let hash = tx_hash.clone();
                    return Task::future(async move {
                        let api = ApiClient::new(network.api_base_url(), network.as_str());
                        match api.get_transactions(vec![hash.clone()]).await {
                            Ok(data) => {
                                if let Some(first) = data.transactions.first() {
                                    let _parsed = parse_transaction(first);
                                    Message::TxLoaded(first.clone())
                                } else {
                                    Message::TxLoadFailed("Transaction not found".to_string())
                                }
                            }
                            Err(e) => Message::TxLoadFailed(e),
                        }
                    });
                }
            }
            Task::none()
        }

        Message::SetNetwork(network) => {
            state.network = network;
            let page = state.current_page.clone();
            update(Message::NavigateTo(page), state)
        }

        Message::SearchInput(val) => {
            state.search_query = val;
            Task::none()
        }

        Message::SearchSubmit => {
            let query = std::mem::take(&mut state.search_query);
            if query.trim().is_empty() {
                return Task::none();
            }
            let (page, network) = detect_search_target(&query);
            state.network = network;
            update(Message::NavigateTo(page), state)
        }

        Message::LoadBlocks => {
            state.blocks_loading = true;
            state.blocks_error = None;
            let network = state.network;
            Task::future(async move {
                let api = ApiClient::new(network.api_base_url(), network.as_str());
                match api.get_blocks(Some(80), Some(true), None, None).await {
                    Ok(data) => Message::BlocksLoaded(data.blocks),
                    Err(e) => Message::LoadBlocksFailed(e),
                }
            })
        }

        Message::BlocksLoaded(blocks) => {
            state.blocks = blocks;
            if let Some(last) = state.blocks.last() {
                state.blocks_resume_height = Some(last.block_height.saturating_sub(1));
            }
            state.blocks_loading = false;
            if state.blocks.len() < 80 {
                state.blocks_has_more = false;
            }
            Task::none()
        }

        Message::LoadBlocksFailed(err) => {
            state.blocks_error = Some(err);
            state.blocks_loading = false;
            Task::none()
        }

        Message::LoadMoreBlocks => {
            if !state.blocks_has_more || state.loading_more_blocks {
                return Task::none();
            }
            state.loading_more_blocks = true;
            let network = state.network;
            let resume = state.blocks_resume_height;
            Task::future(async move {
                let api = ApiClient::new(network.api_base_url(), network.as_str());
                match api.get_blocks(Some(80), Some(true), resume, None).await {
                    Ok(data) => {
                        let new_blocks = data.blocks;
                        let has_more = new_blocks.len() >= 80;
                        let resume_height = new_blocks.last().map(|b| b.block_height.saturating_sub(1));
                        Message::AppendBlocks(new_blocks, has_more, resume_height)
                    }
                    Err(_) => Message::AppendBlocks(vec![], false, None),
                }
            })
        }

        Message::AppendBlocks(new_blocks, has_more, resume_height) => {
            if new_blocks.is_empty() || new_blocks.len() < 80 {
                state.blocks_has_more = false;
            } else {
                state.blocks_has_more = has_more;
            }
            if let Some(rh) = resume_height {
                state.blocks_resume_height = Some(rh);
            }
            state.blocks.extend(new_blocks);
            state.loading_more_blocks = false;
            Task::none()
        }

        Message::LoadAccount(account_id) => {
            state.account_loading = true;
            let network = state.network;
            let acc = account_id.clone();
            Task::future(async move {
                let api = ApiClient::new(network.api_base_url(), network.as_str());
                let filters = AccountFilters::default();
                match api.get_account(&acc, &filters, None, Some(80)).await {
                    Ok(data) => {
                        let hashes: Vec<String> =
                            data.account_txs.iter().map(|t| t.transaction_hash.clone()).collect();
                        let parsed = fetch_and_parse_tx_details(&api, &hashes).await;
                        let parsed_map: HashMap<String, _> = parsed.into_iter().collect();
                        Message::AccountLoaded(data.account_txs, parsed_map, data.txs_count)
                    }
                    Err(_) => Message::AccountLoaded(vec![], HashMap::new(), 0),
                }
            })
        }

        Message::AccountLoaded(txs, parsed, count) => {
            state.account_txs = txs;
            state.account_parsed = parsed;
            state.account_txs_count = count;
            state.account_loading = false;
            state.account_has_more = state.account_txs.len() >= 80;
            Task::none()
        }

        Message::LoadMoreAccount => {
            if !state.account_has_more || state.loading_more_account {
                return Task::none();
            }
            state.loading_more_account = true;
            let network = state.network;
            let account_id = match &state.current_page {
                Page::Account(a) => a.clone(),
                _ => return Task::none(),
            };
            let token = state.account_resume_token.clone();
            Task::future(async move {
                let api = ApiClient::new(network.api_base_url(), network.as_str());
                let filters = AccountFilters::default();
                match api.get_account(&account_id, &filters, token.as_deref(), Some(80)).await {
                    Ok(data) => {
                        let hashes: Vec<String> =
                            data.account_txs.iter().map(|t| t.transaction_hash.clone()).collect();
                        let parsed = fetch_and_parse_tx_details(&api, &hashes).await;
                        let parsed_map: HashMap<String, _> = parsed.into_iter().collect();
                        Message::AppendAccount(data.account_txs, parsed_map)
                    }
                    Err(_) => Message::AppendAccount(vec![], HashMap::new()),
                }
            })
        }

        Message::AppendAccount(new_txs, new_parsed) => {
            let count = new_txs.len();
            state.account_txs.extend(new_txs);
            state.account_parsed.extend(new_parsed);
            state.loading_more_account = false;
            state.account_has_more = count >= 80;
            Task::none()
        }

        Message::LoadBlock(block_id) => {
            state.block_loading = true;
            state.block_error = None;
            let network = state.network;
            let bid = block_id.clone();
            Task::future(async move {
                let api = ApiClient::new(network.api_base_url(), network.as_str());
                let block_identifier = if let Ok(height) = bid.parse::<u64>() {
                    BlockId::Height(height)
                } else {
                    BlockId::Hash(bid)
                };
                match api.get_block(block_identifier, true).await {
                    Ok(data) => Message::BlockLoaded(data),
                    Err(e) => Message::BlockLoadFailed(e),
                }
            })
        }

        Message::BlockLoaded(data) => {
            state.block_detail = Some(data);
            state.block_loading = false;
            Task::none()
        }

        Message::BlockLoadFailed(err) => {
            state.block_error = Some(err);
            state.block_loading = false;
            Task::none()
        }

        Message::LoadTx(tx_hash) => {
            state.tx_loading = true;
            state.tx_error = None;
            let network = state.network;
            let hash = tx_hash.clone();
            Task::future(async move {
                let api = ApiClient::new(network.api_base_url(), network.as_str());
                match api.get_transactions(vec![hash]).await {
                    Ok(data) => {
                        if let Some(first) = data.transactions.first() {
                            Message::TxLoaded(first.clone())
                        } else {
                            Message::TxLoadFailed("Not found".to_string())
                        }
                    }
                    Err(e) => Message::TxLoadFailed(e),
                }
            })
        }

        Message::TxLoaded(tx) => {
            let parsed = parse_transaction(&tx);
            state.tx_parsed = Some(parsed);
            state.tx_detail = Some(tx);
            state.tx_loading = false;
            Task::none()
        }

        Message::TxLoadFailed(err) => {
            state.tx_error = Some(err);
            state.tx_loading = false;
            Task::none()
        }

        Message::Tick => Task::none(),
    }
}

// =========================================
// VIEW
// =========================================

pub fn view<'a>(state: &AppState) -> Element<'a, Message> {
    // Extract ALL data we need for the view FIRST, before borrowing state again
    let network = state.network;
    let current_page = state.current_page.clone();
    let is_home = matches!(current_page, Page::Home);
    let is_block = matches!(current_page, Page::Block(_));
    let is_tx = matches!(current_page, Page::Tx(_));
    
    // Clone the search value BEFORE we start building widgets
    let search_value = state.search_query.clone();
    
    // Clone other state needed for page content
    let blocks = state.blocks.clone();
    let blocks_loading = state.blocks_loading;
    let blocks_error = state.blocks_error.clone();
    let blocks_has_more = state.blocks_has_more;
    let loading_more_blocks = state.loading_more_blocks;
    
    let account_loading = state.account_loading;
    let account_txs = state.account_txs.clone();
    let account_parsed = state.account_parsed.clone();
    let account_txs_count = state.account_txs_count;
    let account_has_more = state.account_has_more;
    let loading_more_account = state.loading_more_account;
    
    let block_loading = state.block_loading;
    let block_error = state.block_error.clone();
    let block_detail = state.block_detail.clone();
    
    let tx_loading = state.tx_loading;
    let tx_error = state.tx_error.clone();
    let tx_detail = state.tx_detail.clone();
    let tx_parsed = state.tx_parsed.clone();
    
    // Get page content - now we can use the cloned data without borrow conflicts
    let content = match &current_page {
        Page::Home => {
            HomePage::view(blocks, blocks_loading, blocks_error, blocks_has_more, loading_more_blocks)
        }
        Page::Account(id) => {
            AccountPage::view_content(id, account_loading, account_txs, account_parsed, account_txs_count, account_has_more, loading_more_account)
        }
        Page::Block(id) => {
            BlockPage::view_content(id, block_loading, block_error, block_detail)
        }
        Page::Tx(id) => {
            TxPage::view_content(id, tx_loading, tx_error, tx_detail, tx_parsed)
        }
    };

    // Build navbar - search_value is already owned, safe to use
    let navbar = container(
        row![
            // Logo
            container(
                Text::new("STICKYEXPLORER")
                    .size(18)
                    .color(Color::from_rgb(0.9, 0.9, 0.9)),
            )
            .padding(iced::Padding::from([0.0, 12.0])),

            // Home link only
            nav_link("Home", is_home),

            Space::new().width(Length::Fixed(20.0)),

            // Search
            SearchBar::view(
                &state.search,
                search_value.clone(),
                Message::SearchInput,
                || Message::SearchSubmit,
            ),

            Space::new().width(Length::Fixed(20.0)),

            // Network toggle (single button)
            network_toggle(
                &state.navbar.network_buttons,
                network,
                |n| Message::SetNetwork(n),
            ),
        ]
        .align_y(Vertical::Center)
        .spacing(6),
    )
    .height(Length::Fixed(52.0))
    .padding(iced::Padding::from([0.0, 12.0]));

    container(
        column![navbar, content]
            .spacing(0)
            .align_x(Horizontal::Center),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn nav_link(label: &'static str, is_active: bool) -> Element<'static, Message> {
    button(
        Text::new(label)
            .size(13)
            .color(if is_active {
                Color::from_rgb(0.3, 0.7, 1.0)
            } else {
                Color::from_rgb(0.6, 0.6, 0.6)
            }),
    )
    .padding(iced::Padding::from([4.0, 10.0]))
    .on_press(Message::NavigateTo(Page::Home))
    .into()
}
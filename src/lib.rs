// stickyexplorer
// =========================================
pub mod api;
pub mod pages {
    pub mod page_account_detail;
    pub mod page_block_detail;
    pub mod page_home;
    pub mod page_tx_detail;
    pub mod route;
}
pub mod components {
    pub mod button_network;
    pub mod search_bar;
    pub mod ui;
    pub mod widgets;
}
pub mod utils;
pub mod utils_web;
pub mod utils_iced;
pub mod icons;
// Iced desktop UI modules — desktop/non-wasm only
#[cfg(not(target_arch = "wasm32"))]
pub mod iced_pages;
#[cfg(not(target_arch = "wasm32"))]
pub mod iced_components;
// =========================================

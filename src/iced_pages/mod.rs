// iced_pages/mod.rs
// =========================================
// Iced page views and app state
// =========================================

pub mod app;
pub mod home_page;
pub mod account_page;
pub mod block_page;
pub mod tx_page;
pub mod iced_app;

pub use app::{AppState, Message, Page};

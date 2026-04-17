// utils_web/mod.rs
// =========================================
// Web/WASM-specific utils (Dioxus web platform)
// =========================================
// These modules use web_sys, wasm_bindgen, or #[cfg(feature = "web")]
// and should NOT be compiled into the iced desktop binary.

pub mod fetch_transactions;
pub mod network;
pub mod tx_cache;
// =========================================

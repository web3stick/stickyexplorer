// utils/mod.rs
// =========================================
// Shared/pure utility modules (no platform-specific deps)
// =========================================
// These modules contain no web_sys, wasm_bindgen, or iced-specific code
// and can be used by both web (Dioxus) and desktop (Iced) platforms.

// Parsing / data extraction (pure logic)
pub mod format;
pub mod parse_action;
pub mod extract_transfers;
pub mod parse_transaction;
pub mod highlight_json;
// =========================================

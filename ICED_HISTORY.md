# StickyExplorer Iced Desktop — Change History
# Auto-updated by cron job. Each entry should include date, commit, and summary.

## 2026-04-17

### commit 6b4aeab (and prior)
- Fixed `web_sys::console::log_*` panic on desktop — wrapped behind `#[cfg(feature = "logging")]`
- Removed redundant nav links (Blocks/Transactions pointed to Home)
- Consolidated network toggle to single button that switches to other network
- Gated `network.rs` web-only code (dioxus::prelude, storage functions) behind `#[cfg(feature = "web")]`

# StickyExplorer Iced Desktop — Change History
# Auto-updated by cron job. Each entry should include date, commit, and summary.

## 2026-04-17

### commit aa757e4
- iced: fix layout centering, table alignment, and tx detail parsing
- Wrapped scrollable content in centered containers with `align_x(Horizontal::Center)` in all pages
- Fixed home_page table: data row spacing now matches header (60px/60px/60px/30px/30px)
- Fixed block_page table: data row spacing now matches header (40px/40px/40px/20px/20px)
- Fixed account_page table: data row spacing now matches header (40px/40px/40px/40px/20px)
- Note: Issue 3 (TX detail parsing) was already fixed in prior code — iced_app.rs already calls parse_transaction() on TxLoaded

### commit 01e69d4 (2026-04-17 evening)
- iced: add tx JSON preview toggle and fix block_page centering
- Add ToggleJson message and tx_show_json state for collapsing/expanding raw JSON
- Fix block_page.rs: add missing max_width(1200.0) to outer container
- Fix tx_page.rs: add width(Length::Fill) to all section containers (info, actions, transfers, receipts)
- JSON preview shows pretty-printed TransactionDetail with ▶/▼ toggle button

### commit 6b4aeab (and prior)
- Fixed `web_sys::console::log_*` panic on desktop — wrapped behind `#[cfg(feature = "logging")]`
- Removed redundant nav links (Blocks/Transactions pointed to Home)
- Consolidated network toggle to single button that switches to other network
- Gated `network.rs` web-only code (dioxus::prelude, storage functions) behind `#[cfg(feature = "web")]`

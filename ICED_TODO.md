# StickyExplorer Iced Desktop — Focus List
# Edit this file to prioritize what the cron job works on next.
# Format: `- [ ] Description` for pending, `- [x]` for done.

## Priority Issues

- [x] Content not centered/filling width — tables and info cards left-aligned with wasted space
- [x] Table headers not aligned with data columns — header spacing doesn't match row spacing
- [x] TX page doesn't fetch signer/receiver details — ParsedTx not created from TransactionDetail
- [ ] **Content not filling full width**: `.max_width(1200.0)` made it worse — narrow and centered. Fix: remove `max_width`, focus on making inner rows/columns stretch to fill available space (use `Width::Fill` for spacers, or wrap inner content in `width(Length::Fill)` containers)
- [ ] **JSON preview toggle on TX page**

## Lower Priority / Future

- [ ] Clean up unused import warnings across iced_pages/ and iced_components/
- [ ] Add more transaction details to TX page (actions, transfers sections)
- [ ] Improve error states with retry buttons
- [ ] Add loading skeletons instead of text "Loading..."

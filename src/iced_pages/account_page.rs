// iced_pages/account_page.rs
// =========================================
// Account detail page for Iced
// =========================================
use crate::iced_pages::{Message, Page};
use crate::api::types::AccountTx;
use crate::utils::parse_transaction::ParsedTx;
use crate::utils::format::{format_time_ago, truncate_middle};
use std::collections::HashMap;
use iced::{Color, Element, Length, Alignment};
use iced::alignment::Vertical;
use iced_widget::{button, container, scrollable, Text, Column, Row, Space};

pub struct AccountPage;

impl AccountPage {
    /// View function that takes individual state pieces instead of full AppState
    pub fn view_content(
        account_id: &str,
        loading: bool,
        txs: Vec<AccountTx>,
        parsed: HashMap<String, ParsedTx>,
        txs_count: u64,
        has_more: bool,
        loading_more: bool,
    ) -> Element<'static, Message> {
        let account_id = account_id.to_string();
        let inner = if loading && txs.is_empty() {
            // Loading state
            Column::new()
                .spacing(8)
                .padding(20)
                .push(Row::new()
                    .push(Text::new("Account: ").size(20).color(Color::from_rgb(0.9, 0.9, 0.95)))
                    .push(Text::new(account_id.clone()).size(16).color(Color::from_rgb(0.7, 0.95, 0.7)))
                    .align_y(Vertical::Center))
                .push(container(
                    Text::new(format!("Loading {}...", account_id.clone())).color(Color::from_rgb(0.5, 0.5, 0.5)),
                ).width(Length::Fill).padding(40))
        } else if txs.is_empty() {
            // Empty state
            Column::new()
                .spacing(8)
                .padding(20)
                .push(Row::new()
                    .push(Text::new("Account: ").size(20).color(Color::from_rgb(0.9, 0.9, 0.95)))
                    .push(Text::new(account_id).size(16).color(Color::from_rgb(0.7, 0.95, 0.7)))
                    .align_y(Vertical::Center))
                .push(container(
                    Text::new("No transactions found").color(Color::from_rgb(0.5, 0.5, 0.5)),
                ).width(Length::Fill).padding(40))
        } else {
            // Success state - build the full content
            let mut col = Column::new().spacing(8).padding(20);

            col = col.push(
                Row::new()
                    .push(Text::new("Account: ").size(20).color(Color::from_rgb(0.9, 0.9, 0.95)))
                    .push(Text::new(account_id).size(16).color(Color::from_rgb(0.7, 0.95, 0.7)))
                    .align_y(Vertical::Center),
            );

            if txs_count > 0 {
                col = col.push(
                    Text::new(format!("Transactions ({})", txs_count))
                        .size(14)
                        .color(Color::from_rgb(0.5, 0.5, 0.5)),
                );
            }

            // Header
            col = col.push(
                container(
                    Row::new()
                        .push(Text::new("Tx Hash").size(12).color(Color::from_rgb(0.5, 0.5, 0.6)))
                        .push(Space::new().width(Length::Fixed(40.0)))
                        .push(Text::new("Time").size(12).color(Color::from_rgb(0.5, 0.5, 0.6)))
                        .push(Space::new().width(Length::Fixed(40.0)))
                        .push(Text::new("Signer").size(12).color(Color::from_rgb(0.5, 0.5, 0.6)))
                        .push(Space::new().width(Length::Fixed(40.0)))
                        .push(Text::new("Receiver").size(12).color(Color::from_rgb(0.5, 0.5, 0.6)))
                        .push(Space::new().width(Length::Fixed(40.0)))
                        .push(Text::new("Action").size(12).color(Color::from_rgb(0.5, 0.5, 0.6)))
                        .push(Space::new().width(Length::Fixed(20.0)))
                        .push(Text::new("Status").size(12).color(Color::from_rgb(0.5, 0.5, 0.6)))
                        .spacing(8),
                )
                .padding(iced::Padding::from([4.0, 12.0])),
            );
            col = col.push(Space::new().height(Length::Fixed(4.0)));

            // Build all rows first, then add them
            let mut rows = Vec::new();
            for atx in txs {
                if let Some(p) = parsed.get(&atx.transaction_hash) {
                    let is_success = p.is_success;
                    let onclick = Message::NavigateTo(Page::Tx(atx.transaction_hash.clone()));

                    let tx_hash = truncate_middle(&atx.transaction_hash, 12);
                    let time_str = format_time_ago(&atx.tx_block_timestamp);
                    let signer = truncate_middle(&p.signer_id, 16);
                    let receiver = truncate_middle(&p.receiver_id, 16);
                    let action_str = p.actions.first().map(|a| {
                        let mut s = a.action_type.clone();
                        if let Some(ref m) = a.method_name {
                            s.push_str(&format!("::{}", m));
                        }
                        s
                    }).unwrap_or_else(|| "Unknown".to_string());

                    let status_text = match is_success {
                        Some(true) => "✓",
                        Some(false) => "✗",
                        None => "⏳",
                    };
                    let status_color = match is_success {
                        Some(true) => Color::from_rgb(0.2, 0.8, 0.2),
                        Some(false) => Color::from_rgb(0.8, 0.2, 0.2),
                        None => Color::from_rgb(0.5, 0.5, 0.3),
                    };

                    let row = button(
                        Row::new()
                            .push(Text::new(tx_hash).size(12).color(Color::from_rgb(0.7, 0.95, 0.7)))
                            .push(Space::new().width(Length::Fixed(20.0)))
                            .push(Text::new(time_str).size(13).color(Color::from_rgb(0.9, 0.9, 0.95)))
                            .push(Space::new().width(Length::Fixed(20.0)))
                            .push(Text::new(signer).size(13).color(Color::from_rgb(0.9, 0.9, 0.95)))
                            .push(Space::new().width(Length::Fixed(20.0)))
                            .push(Text::new(receiver).size(13).color(Color::from_rgb(0.9, 0.9, 0.95)))
                            .push(Space::new().width(Length::Fixed(20.0)))
                            .push(Text::new(action_str).size(12).color(Color::from_rgb(0.6, 0.6, 0.8)))
                            .push(Space::new().width(Length::Fixed(20.0)))
                            .push(Text::new(status_text).size(14).color(status_color))
                            .spacing(8)
                            .align_y(Vertical::Center),
                    )
                    .width(Length::Fill)
                    .on_press(onclick);

                    rows.push(container(row).padding(iced::Padding::from([8.0, 12.0])));
                }
            }
            
            for row_widget in rows {
                col = col.push(row_widget);
            }

            if has_more && !loading {
                col = col.push(Space::new().height(Length::Fixed(8.0)));
                let label = if loading_more { "LOADING..." } else { "LOAD MORE" };
                col = col.push(
                    container(
                        button(Text::new(label))
                        .padding(iced::Padding::from([8.0, 20.0]))
                        .on_press(Message::LoadMoreAccount),
                    )
                    .width(Length::Fill)
                    .align_x(Alignment::Center),
                );
            }

            col
        };

        scrollable(inner)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn mono(s: &str) -> Text {
    Text::new(s)
        .size(12)
        .color(Color::from_rgb(0.7, 0.95, 0.7))
}
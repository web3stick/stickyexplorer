// iced_pages/block_page.rs
// =========================================
// Block detail page for Iced
// =========================================
use crate::iced_pages::{Message, Page};
use crate::iced_pages::app::{label_text, mono_text, value_text};
use crate::api::types::BlockDetailResponse;
use crate::utils::format::{format_gas_amount, format_time_ago, truncate_middle};
use iced::{Color, Element, Length};
use iced::alignment::{Horizontal, Vertical};
use iced_widget::{button, container, scrollable, Column, Container, Row, Space, Text};

pub struct BlockPage;

/// Create a clickable inline link using a flat button style.
/// Uses a transparent style to look like a text link.
fn link_text(text: String, color: Color, target: Message) -> Element<'static, Message> {
    button(Text::new(text).size(12).color(color))
        .padding(iced::Padding::from([0.0, 0.0]))
        .style(move |_: &iced::Theme, _: iced_widget::button::Status| {
            iced_widget::button::Style {
                shadow: iced::Shadow::default(),
                border: iced::Border::default(),
                background: None,
                text_color: color,
                snap: false,
            }
        })
        .on_press(target)
        .into()
}

/// Create a clickable green hash-style link (for tx hash)
fn tx_link(text: String, tx_hash: String) -> Element<'static, Message> {
    link_text(text, Color::from_rgb(0.5, 0.9, 0.5), Message::NavigateTo(Page::Tx(tx_hash)))
}

/// Create a clickable blue account-style link (for signer/receiver)
fn account_link(text: String, account_id: String) -> Element<'static, Message> {
    link_text(text, Color::from_rgb(0.6, 0.75, 1.0), Message::NavigateTo(Page::Account(account_id)))
}

/// Create a clickable block height link
fn block_height_link(text: String, block_id: String) -> Element<'static, Message> {
    link_text(text, Color::from_rgb(0.5, 0.9, 0.5), Message::NavigateTo(Page::Block(block_id)))
}

impl BlockPage {
    /// View function that takes individual state pieces instead of full AppState
    pub fn view_content(
        block_id: &str,
        loading: bool,
        error: Option<String>,
        block_opt: Option<BlockDetailResponse>,
    ) -> Element<'static, Message> {
        let inner = if let Some(err) = error {
            // Error state
            let mut col = Column::new().spacing(8).padding(20);
            col = col.push(
                Row::new()
                    .push(iced_widget::Text::new("Block #").size(18).color(Color::from_rgb(0.9, 0.9, 0.95)))
                    .push(mono_text(block_id))
                    .align_y(Vertical::Center),
            );
            col = col.push(
                Container::new(
                    Text::new(format!("Error: {}", err)).color(Color::from_rgb(0.9, 0.2, 0.2)),
                )
                .width(Length::Fill)
                .padding(20),
            );
            col
        } else if loading || block_opt.is_none() {
            // Loading state
            let mut col = Column::new().spacing(8).padding(20);
            col = col.push(
                Row::new()
                    .push(iced_widget::Text::new("Block #").size(18).color(Color::from_rgb(0.9, 0.9, 0.95)))
                    .push(mono_text(block_id))
                    .align_y(Vertical::Center),
            );
            col = col.push(
                Container::new(
                    Text::new("Loading block...").color(Color::from_rgb(0.5, 0.5, 0.5)),
                )
                .width(Length::Fill)
                .padding(40),
            );
            col
        } else {
            // Success state
            let block = block_opt.unwrap();
            let b = block.block;
            let txs = block.block_txs.clone();

            let mut col = Column::new().spacing(8).padding(20);

            col = col.push(
                Row::new()
                    .push(iced_widget::Text::new("Block #").size(18).color(Color::from_rgb(0.9, 0.9, 0.95)))
                    .push(mono_text(block_id))
                    .align_y(Vertical::Center),
            );

            // Block info card
            let mut info_col = Column::new().spacing(8);

            // Hash
            let block_hash = truncate_middle(&b.block_hash, 20);
            info_col = info_col.push(
                Row::new()
                    .push(label_text("Hash"))
                    .push(Space::new().width(Length::Fixed(12.0)))
                    .push(mono_text(&block_hash))
                    .spacing(8)
                    .align_y(Vertical::Center),
            );

            // Time
            let block_time = format_time_ago(&b.block_timestamp);
            info_col = info_col.push(
                Row::new()
                    .push(label_text("Time"))
                    .push(Space::new().width(Length::Fixed(12.0)))
                    .push(value_text(&block_time))
                    .spacing(8)
                    .align_y(Vertical::Center),
            );

            // Author
            let block_author = truncate_middle(&b.author_id, 24);
            info_col = info_col.push(
                Row::new()
                    .push(label_text("Author"))
                    .push(Space::new().width(Length::Fixed(12.0)))
                    .push(account_link(block_author.clone(), b.author_id.clone()))
                    .spacing(8)
                    .align_y(Vertical::Center),
            );

            // Epoch ID
            let epoch_id = truncate_middle(&b.epoch_id, 16);
            info_col = info_col.push(
                Row::new()
                    .push(label_text("Epoch ID"))
                    .push(Space::new().width(Length::Fixed(12.0)))
                    .push(value_text(&epoch_id))
                    .spacing(8)
                    .align_y(Vertical::Center),
            );

            // Prev Block Height
            if let Some(prev_h) = b.prev_block_height {
                let prev_str = prev_h.to_string();
                info_col = info_col.push(
                    Row::new()
                        .push(label_text("Prev Block Height"))
                        .push(Space::new().width(Length::Fixed(12.0)))
                        .push(block_height_link(prev_str.clone(), prev_h.to_string()))
                        .spacing(8)
                        .align_y(Vertical::Center),
                );
            }

            // Transactions
            let num_tx_str = b.num_transactions.to_string();
            info_col = info_col.push(
                Row::new()
                    .push(label_text("Transactions"))
                    .push(Space::new().width(Length::Fixed(12.0)))
                    .push(value_text(&num_tx_str))
                    .spacing(8)
                    .align_y(Vertical::Center),
            );

            // Receipts
            let num_rec_str = b.num_receipts.to_string();
            info_col = info_col.push(
                Row::new()
                    .push(label_text("Receipts"))
                    .push(Space::new().width(Length::Fixed(12.0)))
                    .push(value_text(&num_rec_str))
                    .spacing(8)
                    .align_y(Vertical::Center),
            );

            // Gas Used
            let gas_used = format_gas_amount(b.gas_burnt.parse().unwrap_or(0));
            info_col = info_col.push(
                Row::new()
                    .push(label_text("Gas Used"))
                    .push(Space::new().width(Length::Fixed(12.0)))
                    .push(value_text(&gas_used))
                    .spacing(8)
                    .align_y(Vertical::Center),
            );

            // Gas Price
            let gas_price = format_near_amount(&b.gas_price);
            info_col = info_col.push(
                Row::new()
                    .push(label_text("Gas Price"))
                    .push(Space::new().width(Length::Fixed(12.0)))
                    .push(value_text(&gas_price))
                    .spacing(8)
                    .align_y(Vertical::Center),
            );

            // Tokens Burnt
            let tokens_burnt = format_near_amount(&b.tokens_burnt);
            info_col = info_col.push(
                Row::new()
                    .push(label_text("Tokens Burnt"))
                    .push(Space::new().width(Length::Fixed(12.0)))
                    .push(value_text(&tokens_burnt))
                    .spacing(8)
                    .align_y(Vertical::Center),
            );

            // Chunks
            let chunks_str = b.chunks_included.to_string();
            info_col = info_col.push(
                Row::new()
                    .push(label_text("Chunks"))
                    .push(Space::new().width(Length::Fixed(12.0)))
                    .push(value_text(&chunks_str))
                    .spacing(8)
                    .align_y(Vertical::Center),
            );

            // Protocol Version
            let proto_str = b.protocol_version.to_string();
            info_col = info_col.push(
                Row::new()
                    .push(label_text("Protocol Version"))
                    .push(Space::new().width(Length::Fixed(12.0)))
                    .push(value_text(&proto_str))
                    .spacing(8)
                    .align_y(Vertical::Center),
            );

            col = col.push(Container::new(info_col).padding(16));
            col = col.push(Space::new().height(Length::Fixed(16.0)));

            // Transactions
            if !txs.is_empty() {
                col = col.push(
                    Text::new(format!("Transactions ({})", b.num_transactions))
                        .size(16)
                        .color(Color::from_rgb(0.8, 0.8, 0.9)),
                );
                col = col.push(Space::new().height(Length::Fixed(8.0)));

                // Header row
                col = col.push(
                    Container::new(
                        Row::new()
                            .push(label_text("Tx Hash"))
                            .push(Space::new().width(Length::Fixed(40.0)))
                            .push(label_text("Time"))
                            .push(Space::new().width(Length::Fixed(40.0)))
                            .push(label_text("Signer"))
                            .push(Space::new().width(Length::Fixed(40.0)))
                            .push(label_text("Receiver"))
                            .push(Space::new().width(Length::Fixed(20.0)))
                            .push(label_text("Gas"))
                            .push(Space::new().width(Length::Fixed(20.0)))
                            .push(label_text("Status"))
                            .spacing(8),
                    )
                    .padding(iced::Padding::from([4.0, 12.0])),
                );

                for tx in txs {
                    let hash_display = truncate_middle(&tx.transaction_hash, 12);
                    let time_str = format_time_ago(&tx.tx_block_timestamp);
                    let gas_str = format_gas_amount(tx.gas_burnt);
                    let status = if tx.is_success { "✓" } else { "✗" };
                    let is_success = tx.is_success;
                    let signer_str = truncate_middle(&tx.signer_id, 16);
                    let receiver_str = truncate_middle(&tx.receiver_id, 16);

                    let status_color = if is_success {
                        Color::from_rgb(0.2, 0.8, 0.2)
                    } else {
                        Color::from_rgb(0.8, 0.2, 0.2)
                    };

                    // Build row with individually clickable links
                    let row = Row::new()
                        .push(tx_link(hash_display.clone(), tx.transaction_hash.clone()))
                        .push(Space::new().width(Length::Fixed(40.0)))
                        .push(value_text(&time_str))
                        .push(Space::new().width(Length::Fixed(40.0)))
                        .push(account_link(signer_str.clone(), tx.signer_id.clone()))
                        .push(Space::new().width(Length::Fixed(40.0)))
                        .push(account_link(receiver_str.clone(), tx.receiver_id.clone()))
                        .push(Space::new().width(Length::Fixed(20.0)))
                        .push(mono_text(&gas_str))
                        .push(Space::new().width(Length::Fixed(20.0)))
                        .push(Text::new(status).size(14).color(status_color))
                        .spacing(8)
                        .align_y(Vertical::Center);

                    col = col.push(
                        Container::new(row)
                            .padding(iced::Padding::from([8.0, 12.0])),
                    );
                }
            }

            col
        };

        container(
            scrollable(inner)
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .align_x(Horizontal::Center)
        .into()
    }
}

fn format_near_amount(yocto: &str) -> String {
    format!("{} NEAR", crate::utils::format::format_near_amount(yocto))
}

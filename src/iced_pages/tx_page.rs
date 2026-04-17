// iced_pages/tx_page.rs
// =========================================
// Transaction detail page for Iced
// =========================================
use crate::iced_pages::Message;
use crate::iced_pages::Page;
use crate::iced_pages::app::{label_text, mono_text, value_text};
use crate::api::types::TransactionDetail;
use crate::utils::parse_transaction::ParsedTx;
use crate::utils::format::{format_gas_amount, format_time_ago, format_near_amount, truncate_middle};
use crate::utils::extract_transfers::TokenType;
use iced::{Color, Element, Length};
use iced::alignment::{Horizontal, Vertical};
use iced_widget::{button, container, scrollable, Column, Container, Row, Space, Text};

pub struct TxPage;

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

/// Create a clickable blue account-style link (for signer/receiver)
fn account_link(text: String, account_id: String) -> Element<'static, Message> {
    link_text(text, Color::from_rgb(0.6, 0.75, 1.0), Message::NavigateTo(Page::Account(account_id)))
}

/// Create a clickable block height link
fn block_link(text: String, block_id: String) -> Element<'static, Message> {
    link_text(text, Color::from_rgb(0.5, 0.9, 0.5), Message::NavigateTo(Page::Block(block_id)))
}

impl TxPage {
    /// View function that takes individual state pieces instead of full AppState
    pub fn view_content(
        _tx_hash: &str,
        loading: bool,
        error: Option<String>,
        tx_opt: Option<TransactionDetail>,
        parsed_opt: Option<ParsedTx>,
    ) -> Element<'static, Message> {
        // Build the result piece by piece
        let inner = if let Some(err) = error {
            // Error state
            let mut col = Column::new().spacing(8).padding(20);
            col = col.push(
                Text::new("Transaction")
                    .size(20)
                    .color(Color::from_rgb(0.9, 0.9, 0.95)),
            );
            col = col.push(
                Container::new(
                    Text::new(format!("Error: {}", err)).color(Color::from_rgb(0.9, 0.2, 0.2)),
                )
                .width(Length::Fill)
                .padding(20),
            );
            col
        } else if loading || tx_opt.is_none() || parsed_opt.is_none() {
            // Loading state
            let mut col = Column::new().spacing(8).padding(20);
            col = col.push(
                Text::new("Transaction")
                    .size(20)
                    .color(Color::from_rgb(0.9, 0.9, 0.95)),
            );
            col = col.push(
                Container::new(
                    Text::new("Loading transaction...").color(Color::from_rgb(0.5, 0.5, 0.5)),
                )
                .width(Length::Fill)
                .padding(40),
            );
            col
        } else {
            // Success state - build the full content
            let ptx = parsed_opt.unwrap();
            let mut col = Column::new().spacing(8).padding(20);

            // Main info card
            let mut info_col = Column::new().spacing(10);

            let hash = truncate_middle(&ptx.hash, 32);
            let signer = truncate_middle(&ptx.signer_id, 24);
            let receiver = truncate_middle(&ptx.receiver_id, 24);
            let block_height = ptx.block_height.to_string();
            let time = format_time_ago(&ptx.timestamp);
            let gas_used = format_gas_amount(ptx.gas_burnt);

            // Hash row
            info_col = info_col.push(
                Row::new()
                    .push(label_text("Hash"))
                    .push(Space::new().width(Length::Fixed(12.0)))
                    .push(mono_text(&hash))
                    .spacing(8)
                    .align_y(Vertical::Center),
            );
            // Signer row with clickable link
            info_col = info_col.push(
                Row::new()
                    .push(label_text("Signer"))
                    .push(Space::new().width(Length::Fixed(12.0)))
                    .push(account_link(signer.clone(), ptx.signer_id.clone()))
                    .spacing(8)
                    .align_y(Vertical::Center),
            );
            // Receiver row with clickable link
            info_col = info_col.push(
                Row::new()
                    .push(label_text("Receiver"))
                    .push(Space::new().width(Length::Fixed(12.0)))
                    .push(account_link(receiver.clone(), ptx.receiver_id.clone()))
                    .spacing(8)
                    .align_y(Vertical::Center),
            );
            // Block row with clickable link
            info_col = info_col.push(
                Row::new()
                    .push(label_text("Block"))
                    .push(Space::new().width(Length::Fixed(12.0)))
                    .push(block_link(block_height.clone(), ptx.block_height.to_string()))
                    .spacing(8)
                    .align_y(Vertical::Center),
            );
            info_col = info_col.push(
                Row::new()
                    .push(label_text("Time"))
                    .push(Space::new().width(Length::Fixed(12.0)))
                    .push(value_text(&time))
                    .spacing(8)
                    .align_y(Vertical::Center),
            );
            info_col = info_col.push(
                Row::new()
                    .push(label_text("Gas Used"))
                    .push(Space::new().width(Length::Fixed(12.0)))
                    .push(value_text(&gas_used))
                    .spacing(8)
                    .align_y(Vertical::Center),
            );

            if let Some(success) = ptx.is_success {
                let status_str = if success { "✓ Success" } else { "✗ Failed" };
                info_col = info_col.push(
                    Row::new()
                        .push(label_text("Status"))
                        .push(Space::new().width(Length::Fixed(12.0)))
                        .push(value_text(status_str))
                        .spacing(8)
                        .align_y(Vertical::Center),
                );
            }

            col = col.push(
                Text::new("Transaction")
                    .size(20)
                    .color(Color::from_rgb(0.9, 0.9, 0.95)),
            );
            col = col.push(Container::new(info_col).padding(16));

            // Actions
            if !ptx.actions.is_empty() {
                col = col.push(Space::new().height(Length::Fixed(12.0)));
                col = col.push(
                    Text::new("Actions")
                        .size(15)
                        .color(Color::from_rgb(0.7, 0.7, 0.9)),
                );
                col = col.push(Space::new().height(Length::Fixed(4.0)));

                let mut actions_col = Column::new().spacing(4);
                for action in ptx.actions {
                    let mut action_str = action.action_type.clone();
                    if let Some(m) = action.method_name {
                        action_str.push_str(&format!("::{}", m));
                    }
                    if let Some(d) = action.deposit {
                        action_str.push_str(&format!(" (deposit: {} NEAR)", format_near_amount(&d)));
                    }
                    actions_col = actions_col.push(
                        Container::new(
                            Text::new(action_str)
                                .size(12)
                                .color(Color::from_rgb(0.8, 0.8, 0.9)),
                        )
                        .padding(iced::Padding::from([4.0, 8.0])),
                    );
                }
                col = col.push(Container::new(actions_col).padding(12));
            }

            // Transfers
            if !ptx.transfers.is_empty() || !ptx.nft_transfers.is_empty() {
                col = col.push(Space::new().height(Length::Fixed(12.0)));
                col = col.push(
                    Text::new("Transfers")
                        .size(15)
                        .color(Color::from_rgb(0.7, 0.7, 0.9)),
                );
                col = col.push(Space::new().height(Length::Fixed(4.0)));

                let mut transfers_col = Column::new().spacing(4);

                for transfer in ptx.transfers {
                    let from_str = transfer.from.as_deref().unwrap_or("(mint)").to_string();
                    let to_str = transfer.to.as_deref().unwrap_or("(burn)").to_string();
                    let token_type_str = match transfer.token_type {
                        TokenType::Near => "NEAR",
                        TokenType::Nep141 => "FT",
                        TokenType::Nep245 => "MT",
                    };
                    let display_amount = if transfer.token_type == TokenType::Near {
                        format!("{} NEAR", format_near_amount(&transfer.amount))
                    } else {
                        format!("{} {}", transfer.amount, token_type_str)
                    };
                    let from_trunc = truncate_middle(&from_str, 16);
                    let to_trunc = truncate_middle(&to_str, 16);

                    transfers_col = transfers_col.push(
                        Container::new(
                            Text::new(format!("{} → {}: {}", from_trunc, to_trunc, display_amount))
                                .size(12)
                                .color(Color::from_rgb(0.7, 0.95, 0.7)),
                        )
                        .padding(iced::Padding::from([4.0, 8.0])),
                    );
                }

                for nft in ptx.nft_transfers {
                    let from_str = nft.from.as_deref().unwrap_or("(mint)").to_string();
                    let to_str = nft.to.as_deref().unwrap_or("(burn)").to_string();
                    let from_trunc = truncate_middle(&from_str, 16);
                    let to_trunc = truncate_middle(&to_str, 16);
                    let contract_trunc = truncate_middle(&nft.contract_id, 16);

                    transfers_col = transfers_col.push(
                        Container::new(
                            Text::new(format!("{} → {}: NFT #{} ({})", from_trunc, to_trunc, nft.token_id, contract_trunc))
                                .size(12)
                                .color(Color::from_rgb(0.7, 0.85, 1.0)),
                        )
                        .padding(iced::Padding::from([4.0, 8.0])),
                    );
                }

                col = col.push(Container::new(transfers_col).padding(12));
            }

            // Receipts
            if !ptx.receipts.is_empty() {
                col = col.push(Space::new().height(Length::Fixed(12.0)));
                col = col.push(
                    Text::new(format!("Receipts ({})", ptx.receipts.len()))
                        .size(15)
                        .color(Color::from_rgb(0.7, 0.7, 0.9)),
                );
                col = col.push(Space::new().height(Length::Fixed(4.0)));

                let mut receipts_col = Column::new().spacing(4);
                for receipt in ptx.receipts {
                    let predecessor = receipt.receipt.predecessor_id.clone();
                    let receiver = receipt.receipt.receiver_id.clone();
                    let gas = receipt.execution_outcome.outcome.gas_burnt;
                    let predecessor_str = truncate_middle(&predecessor, 16);
                    let receiver_str = truncate_middle(&receiver, 16);
                    let gas_str = format_gas_amount(gas);

                    receipts_col = receipts_col.push(
                        Container::new(
                            Row::new()
                                .push(account_link(predecessor_str.clone(), predecessor.clone()))
                                .push(Text::new(" → ").color(Color::from_rgb(0.4, 0.4, 0.4)))
                                .push(account_link(receiver_str.clone(), receiver.clone()))
                                .push(Space::new().width(Length::Fixed(12.0)))
                                .push(mono_text(&gas_str))
                                .align_y(Vertical::Center),
                        )
                        .padding(iced::Padding::from([6.0, 10.0])),
                    );
                }

                col = col.push(Container::new(receipts_col).padding(12));
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

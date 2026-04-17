// iced_pages/home_page.rs
// =========================================
// Home page - Latest blocks list for Iced
// =========================================
use crate::iced_pages::{Message, Page};
use crate::utils::format::{format_gas_amount, format_time_ago, truncate_middle};
use iced::{Color, Element, Length};
use iced::alignment::{Horizontal, Vertical};
use iced_widget::{button, container, scrollable, Text, Column, Row, Space};
use crate::api::types::BlockHeader;

pub struct HomePage;

/// Create a clickable inline link using a flat button style.
/// Takes owned String to avoid lifetime issues.
fn link_text(text: String, color: Color, target: Message) -> Element<'static, Message> {
    button(
        Text::new(text)
            .size(13)
            .color(color)
    )
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

/// Create a clickable green hash-style link (for block height)
fn block_height_link(text: String, block_id: String) -> Element<'static, Message> {
    link_text(text, Color::from_rgb(0.5, 0.9, 0.5), Message::NavigateTo(Page::Block(block_id)))
}

/// Create a clickable blue account-style link (for author)
fn author_link(text: String, account_id: String) -> Element<'static, Message> {
    link_text(text, Color::from_rgb(0.6, 0.75, 1.0), Message::NavigateTo(Page::Account(account_id)))
}

impl HomePage {
    pub fn view(
        blocks: Vec<BlockHeader>,
        loading: bool,
        error: Option<String>,
        loading_more: bool,
        has_more: bool,
    ) -> Element<'static, Message> {
        let mut content_col = Column::new().spacing(4).padding(16);

        if let Some(err) = error {
            content_col = content_col.push(
                container(
                    Text::new(format!("Error: {}", err)).color(Color::from_rgb(0.9, 0.3, 0.3)),
                )
                .width(Length::Fill)
                .padding(20),
            );
        }

        if loading && blocks.is_empty() {
            content_col = content_col.push(
                container(Text::new("Loading blocks...").color(Color::from_rgb(0.5, 0.5, 0.5)))
                    .width(Length::Fill)
                    .padding(40),
            );
        } else if blocks.is_empty() {
            content_col = content_col.push(
                container(Text::new("No blocks available").color(Color::from_rgb(0.5, 0.5, 0.5)))
                    .width(Length::Fill)
                    .padding(40),
            );
        } else {
            // Header row
            content_col = content_col.push({
                let row = Row::new()
                    .push(
                        Text::new("Height")
                            .size(13)
                            .color(Color::from_rgb(0.5, 0.5, 0.5)),
                    )
                    .push(Space::new().width(Length::Fixed(60.0)))
                    .push(
                        Text::new("Time")
                            .size(13)
                            .color(Color::from_rgb(0.5, 0.5, 0.5)),
                    )
                    .push(Space::new().width(Length::Fixed(60.0)))
                    .push(
                        Text::new("Author")
                            .size(13)
                            .color(Color::from_rgb(0.5, 0.5, 0.5)),
                    )
                    .push(Space::new().width(Length::Fixed(60.0)))
                    .push(
                        Text::new("Txns")
                            .size(13)
                            .color(Color::from_rgb(0.5, 0.5, 0.5)),
                    )
                    .push(Space::new().width(Length::Fixed(30.0)))
                    .push(
                        Text::new("Receipts")
                            .size(13)
                            .color(Color::from_rgb(0.5, 0.5, 0.5)),
                    )
                    .push(Space::new().width(Length::Fixed(30.0)))
                    .push(
                        Text::new("Gas Used")
                            .size(13)
                            .color(Color::from_rgb(0.5, 0.5, 0.5)),
                    );
                Element::from(container(row).padding(iced::Padding::from([4.0, 12.0])))
            });
            content_col = content_col.push(Space::new().height(Length::Fixed(4.0)));

            for block in &blocks {
                let height = format!("{}", block.block_height);
                let block_id_str = block.block_hash.clone();
                let time = format_time_ago(&block.block_timestamp);
                let author = truncate_middle(&block.author_id, 16);
                let num_transactions = format!("{}", block.num_transactions);
                let num_receipts = format!("{}", block.num_receipts);
                let gas_used = format_gas_amount(block.gas_burnt.parse::<u64>().unwrap_or(0));

                let row = Row::new()
                    .push(block_height_link(height.clone(), block_id_str.clone()))
                    .push(Space::new().width(Length::Fixed(60.0)))
                    .push(
                        Text::new(time)
                            .size(13)
                            .color(Color::from_rgb(0.9, 0.9, 0.9)),
                    )
                    .push(Space::new().width(Length::Fixed(60.0)))
                    .push(author_link(author.clone(), block.author_id.clone()))
                    .push(Space::new().width(Length::Fixed(60.0)))
                    .push(
                        Text::new(num_transactions)
                            .size(13)
                            .color(Color::from_rgb(0.9, 0.9, 0.9)),
                    )
                    .push(Space::new().width(Length::Fixed(30.0)))
                    .push(
                        Text::new(num_receipts)
                            .size(13)
                            .color(Color::from_rgb(0.9, 0.9, 0.9)),
                    )
                    .push(Space::new().width(Length::Fixed(30.0)))
                    .push(
                        Text::new(gas_used)
                            .size(13)
                            .color(Color::from_rgb(0.7, 0.95, 0.7)),
                    )
                    .align_y(Vertical::Center);

                content_col =
                    content_col.push(container(row).padding(iced::Padding::from([8.0, 12.0])));
            }

            content_col = content_col.push(Space::new().height(Length::Fixed(8.0)));

            // Load more button
            let btn_label = if loading_more {
                "LOADING..."
            } else if has_more {
                "LOAD MORE"
            } else {
                "NO MORE"
            };
            content_col = content_col.push(
                container(
                    button(Text::new(btn_label).color(if has_more {
                        Color::from_rgb(0.7, 0.85, 1.0)
                    } else {
                        Color::from_rgb(0.3, 0.3, 0.3)
                    }))
                    .padding(iced::Padding::from([8.0, 20.0]))
                    .on_press(Message::LoadMoreBlocks),
                )
                .width(Length::Fill)
                .align_x(Horizontal::Center),
            );
        }

        container(
            scrollable(content_col)
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .max_width(1200.0)
        .align_x(Horizontal::Center)
        .into()
    }
}

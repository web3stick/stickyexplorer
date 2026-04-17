// iced_pages/home_page.rs
// =========================================
// Home page - Latest blocks list for Iced
// =========================================
use crate::iced_pages::{AppState, Message, Page};
use crate::iced_pages::app::mono_text;
use crate::utils::format::{format_gas_amount, format_time_ago, truncate_middle};
use iced::{Color, Element, Length};
use iced::alignment::{Horizontal, Vertical};
use iced_widget::{button, container, scrollable, Text, Column, Row, Space};
use crate::api::types::BlockHeader;

pub struct HomePage;

impl HomePage {
    pub fn view(
        blocks: Vec<BlockHeader>,
        loading: bool,
        error: Option<String>,
        has_more: bool,
        loading_more: bool,
    ) -> Element<'static, Message> {
        // Error state
        if let Some(err) = &error {
            return container(
                Text::new(format!("Error loading blocks: {}", err))
                    .color(Color::from_rgb(0.9, 0.2, 0.2)),
            )
            .width(Length::Fill)
            .padding(20)
            .into();
        }

        // Build content column
        let mut content_col = Column::new().spacing(8).padding(20);
        content_col = content_col.push(
            Text::new("Latest Blocks")
                .size(22)
                .color(Color::from_rgb(0.95, 0.95, 1.0)),
        );
        content_col = content_col.push(Space::new().height(Length::Fixed(12.0)));

        if loading && blocks.is_empty() {
            content_col = content_col.push(
                container(
                    Text::new("Loading blocks...").color(Color::from_rgb(0.5, 0.5, 0.5)),
                )
                .width(Length::Fill)
                .padding(40),
            );
        } else if blocks.is_empty() {
            content_col = content_col.push(
                container(
                    Text::new("No blocks available").color(Color::from_rgb(0.5, 0.5, 0.5)),
                )
                .width(Length::Fill)
                .padding(40),
            );
        } else {
            // Header row
            content_col = content_col.push({
                let row = Row::new()
                    .push(Text::new("Height").size(13).color(Color::from_rgb(0.5, 0.5, 0.5)))
                    .push(Space::new().width(Length::Fixed(60.0)))
                    .push(Text::new("Time").size(13).color(Color::from_rgb(0.5, 0.5, 0.5)))
                    .push(Space::new().width(Length::Fixed(60.0)))
                    .push(Text::new("Author").size(13).color(Color::from_rgb(0.5, 0.5, 0.5)))
                    .push(Space::new().width(Length::Fixed(60.0)))
                    .push(Text::new("Txns").size(13).color(Color::from_rgb(0.5, 0.5, 0.5)))
                    .push(Space::new().width(Length::Fixed(30.0)))
                    .push(Text::new("Receipts").size(13).color(Color::from_rgb(0.5, 0.5, 0.5)))
                    .push(Space::new().width(Length::Fixed(30.0)))
                    .push(Text::new("Gas Used").size(13).color(Color::from_rgb(0.5, 0.5, 0.5)));
                Element::from(container(row).padding(iced::Padding::from([4.0, 12.0])))
            });
            content_col = content_col.push(Space::new().height(Length::Fixed(4.0)));

            for block in blocks {
                let height = block.block_height.to_string();
                let time = format_time_ago(&block.block_timestamp);
                let author = truncate_middle(&block.author_id, 20);
                let num_transactions = block.num_transactions.to_string();
                let num_receipts = block.num_receipts.to_string();
                let gas_used = format_gas_amount(block.gas_burnt.parse().unwrap_or(0));
                let onclick = Message::NavigateTo(Page::Block(block.block_height.to_string()));

                let row = Row::new()
                    .push(Text::new(height).size(13).color(Color::from_rgb(0.7, 0.95, 0.7)))
                    .push(Space::new().width(Length::Fixed(20.0)))
                    .push(Text::new(time).size(13).color(Color::from_rgb(0.9, 0.9, 0.9)))
                    .push(Space::new().width(Length::Fixed(20.0)))
                    .push(Text::new(author).size(13).color(Color::from_rgb(0.9, 0.9, 0.9)))
                    .push(Space::new().width(Length::Fixed(20.0)))
                    .push(Text::new(num_transactions).size(13).color(Color::from_rgb(0.9, 0.9, 0.9)))
                    .push(Space::new().width(Length::Fixed(20.0)))
                    .push(Text::new(num_receipts).size(13).color(Color::from_rgb(0.9, 0.9, 0.9)))
                    .push(Space::new().width(Length::Fixed(20.0)))
                    .push(Text::new(gas_used).size(13).color(Color::from_rgb(0.7, 0.95, 0.7)))
                    .align_y(Vertical::Center);

                content_col = content_col.push(
                    container(button(row).width(Length::Fill).on_press(onclick))
                        .padding(iced::Padding::from([8.0, 12.0]))
                );
            }

            content_col = content_col.push(Space::new().height(Length::Fixed(8.0)));

            // Load more button
            let btn_label = if loading_more { "LOADING..." } else if has_more { "LOAD MORE" } else { "NO MORE" };
            content_col = content_col.push(
                container(
                    button(
                        Text::new(btn_label)
                            .color(if has_more {
                                Color::from_rgb(0.7, 0.85, 1.0)
                            } else {
                                Color::from_rgb(0.3, 0.3, 0.3)
                            }),
                    )
                    .padding(iced::Padding::from([8.0, 20.0]))
                    .on_press(Message::LoadMoreBlocks),
                )
                .width(Length::Fill)
                .align_x(Horizontal::Center),
            );
        }

        scrollable(content_col)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

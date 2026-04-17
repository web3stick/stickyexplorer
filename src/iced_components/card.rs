// iced_components/card.rs
// =========================================
// Card component for Iced
// =========================================
use crate::iced_pages::Message;
use iced::{Color, Element, Length};
use iced_widget::container::Container;

// =========================================

pub struct Card;

impl Card {
    pub fn new<'a, T: 'a>(content: Element<'a, Message>) -> Container<'a, Message>
    where
        Message: 'a,
    {
        Container::new(content)
            .width(Length::Fill)
            .padding(16)
    }
}

// =========================================
// A simple labeled row for key-value display
// =========================================
use iced::alignment::Vertical;
use iced_widget::{row, text::Text, Row, Space};

pub struct KeyValue;

impl KeyValue {
    pub fn row<'a>(label: &'a str, value: &'a str) -> Row<'a, Message> {
        row![
            Text::new(label)
                .size(13)
                .color(Color::from_rgb(0.5, 0.5, 0.5)),
            Space::new().width(Length::Fixed(8.0)),
            Text::new(value)
                .size(13)
                .color(Color::from_rgb(0.9, 0.9, 0.9)),
        ]
        .align_y(Vertical::Center)
        .spacing(4)
    }

    pub fn mono_row<'a>(label: &'a str, value: &'a str) -> Row<'a, Message> {
        row![
            Text::new(label)
                .size(13)
                .color(Color::from_rgb(0.5, 0.5, 0.5)),
            Space::new().width(Length::Fixed(8.0)),
            Text::new(value)
                .size(12)
                .color(Color::from_rgb(0.7, 0.95, 0.7)),
        ]
        .align_y(Vertical::Center)
        .spacing(4)
    }
}

// iced_components/search_bar.rs
// =========================================
// Search bar for Iced
// =========================================
use crate::iced_pages::Message;
use iced::{Element, Length};
use iced_widget::{Button, Row, Text, TextInput};

#[derive(Debug, Clone)]
pub struct SearchBarState {
    pub input: (),
    pub button: (),
}

impl SearchBarState {
    pub fn new() -> Self {
        Self {
            input: (),
            button: (),
        }
    }
}

impl Default for SearchBarState {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SearchBar;

impl SearchBar {
    pub fn view(
        _state: &SearchBarState,
        value: String,
        on_change: fn(String) -> Message,
        on_submit: fn() -> Message,
    ) -> Element<'static, Message> {
        let placeholder = "Search tx, block, or account...";

        Row::new()
            .push(
                TextInput::new(placeholder, &value)
                    .width(Length::Fixed(300.0))
                    .padding(iced::Padding::from([8.0, 16.0]))
                    .on_input(on_change)
                    .on_submit(on_submit()),
            )
            .push(
                Button::new(Text::new("GO"))
                    .padding(iced::Padding::from([8.0, 16.0]))
                    .on_press(on_submit()),
            )
            .spacing(4)
            .into()
    }
}

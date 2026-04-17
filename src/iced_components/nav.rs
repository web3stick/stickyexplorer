// iced_components/nav.rs
// =========================================
// Navigation bar for Iced
// =========================================
use crate::iced_pages::Message;
use crate::iced_pages::Page;
use crate::iced_components::SearchBar;
use crate::iced_components::search_bar::SearchBarState;
use crate::iced_components::button::network_toggle;
use crate::iced_components::button::NetworkButtonState;
use crate::utils_iced::network::NetworkId;
use iced::{Color, Length};
use iced::alignment::Vertical;
use iced_widget::{button, row, text::Text, Container, Space};

// =========================================

#[derive(Debug, Clone)]
pub struct NavbarState {
    pub network_buttons: NetworkButtonState,
    pub search: SearchBarState,
}

impl NavbarState {
    pub fn new() -> Self {
        Self {
            network_buttons: NetworkButtonState::new(),
            search: SearchBarState::new(),
        }
    }
}

impl Default for NavbarState {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Navbar;

impl Navbar {
    pub fn view<'a>(
        state: &'a mut NavbarState,
        current_network: NetworkId,
        _active_page: Page,
        search_value: String,
        on_search_change: fn(String) -> Message,
        on_search_submit: fn() -> Message,
        on_network_select: fn(NetworkId) -> Message,
    ) -> Container<'a, Message> {
        let search_bar_state = &mut state.search;
        let network_buttons = &mut state.network_buttons;

        // Logo as clickable link to home
        let logo = button(
            Text::new("STICKYEXPLORER")
                .size(20)
                .color(Color::from_rgb(0.9, 0.9, 0.9)),
        )
        .on_press(Message::NavigateTo(Page::Home));
        Container::new(
            row![
                logo,
                Space::new().width(Length::Fixed(20.0)),
                // Search bar
                SearchBar::view(
                    search_bar_state,
                    search_value,
                    on_search_change,
                    on_search_submit,
                ),
                Space::new().width(Length::Fixed(20.0)),
                // Network toggle
                network_toggle(
                    network_buttons,
                    current_network,
                    on_network_select,
                ),
            ]
            .align_y(Vertical::Center)
            .spacing(10),
        )
        .padding(15)
    }
}

// iced_components/nav.rs
// =========================================
// Navigation bar for Iced
// =========================================
use crate::iced_pages::Message;
use crate::iced_components::SearchBar;
use crate::iced_components::search_bar::SearchBarState;
use crate::iced_components::button::network_toggle;
use crate::iced_components::button::NetworkButtonState;
use crate::utils_iced::network::NetworkId;
use iced::{Color, Element, Length};
use iced::alignment::Vertical;
use iced_widget::{container, row, text::Text, Container, Row, Space};

// =========================================

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
        search_value: String,
        on_search_change: fn(String) -> Message,
        on_search_submit: fn() -> Message,
        on_network_select: fn(NetworkId) -> Message,
        active_page: &'a str,
    ) -> Container<'a, Message> {
        let search_bar_state = &mut state.search;
        let network_buttons = &mut state.network_buttons;

        Container::new(
            row![
                // Logo
                Text::new("STICKYEXPLORER")
                    .size(20)
                    .color(Color::from_rgb(0.9, 0.9, 0.9)),
                Space::new().width(Length::Fixed(20.0)),
                // Nav links
                nav_link("Home", "/", active_page),
                Space::new().width(Length::Fixed(8.0)),
                nav_link("Blocks", "/blocks", active_page),
                Space::new().width(Length::Fixed(8.0)),
                nav_link("Transactions", "/txs", active_page),
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

fn nav_link<'a>(label: &'static str, _route: &'static str, active: &'a str) -> Element<'a, Message> {
    let is_active = match (label, active) {
        ("Home", "home") => true,
        ("Blocks", "blocks") => true,
        ("Transactions", "txs") => true,
        _ => false,
    };

    let text = Text::new(label)
        .size(14)
        .color(if is_active {
            Color::from_rgb(0.3, 0.7, 1.0)
        } else {
            Color::from_rgb(0.7, 0.7, 0.7)
        });

    Container::new(text)
        .padding(iced::Padding::from([0.0, 8.0]))
        .into()
}

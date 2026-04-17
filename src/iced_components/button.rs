// iced_components/button.rs
// =========================================
// Network toggle button for Iced
// =========================================
use crate::iced_pages::Message;
use crate::iced_pages::app::Page;
use crate::utils::network::NetworkId;
use iced::{Color, Element};
use iced_widget::{Button, Row, Text};

#[derive(Debug, Clone)]
pub enum NetworkToggle {
    Mainnet,
    Testnet,
}

impl NetworkToggle {
    pub fn from_network(network: NetworkId) -> Self {
        match network {
            NetworkId::Mainnet => NetworkToggle::Mainnet,
            NetworkId::Testnet => NetworkToggle::Testnet,
        }
    }

    pub fn network_id(&self) -> NetworkId {
        match self {
            NetworkToggle::Mainnet => NetworkId::Mainnet,
            NetworkToggle::Testnet => NetworkId::Testnet,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NetworkButtonState {
    pub mainnet: (),
    pub testnet: (),
}

impl NetworkButtonState {
    pub fn new() -> Self {
        Self {
            mainnet: (),
            testnet: (),
        }
    }
}

impl Default for NetworkButtonState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn network_toggle(
    _state: &NetworkButtonState,
    current_network: NetworkId,
    on_select: fn(NetworkId) -> Message,
) -> Element<'static, Message> {
    let (label, color) = match current_network {
        NetworkId::Mainnet => ("Mainnet", Color::from_rgb(0.2, 0.8, 0.2)),
        NetworkId::Testnet => ("Testnet", Color::from_rgb(0.2, 0.8, 0.2)),
    };

    Button::new(Text::new(label).color(color))
        .padding(iced::Padding::from([4.0, 12.0]))
        .on_press(on_select(current_network.other()))
        .into()
}

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
    let is_mainnet = current_network == NetworkId::Mainnet;

    let mainnet_text = if is_mainnet {
        Text::new("🟢 Mainnet").color(Color::from_rgb(0.2, 0.8, 0.2))
    } else {
        Text::new("Mainnet")
    };

    let testnet_text = if !is_mainnet {
        Text::new("🟢 Testnet").color(Color::from_rgb(0.2, 0.8, 0.2))
    } else {
        Text::new("Testnet")
    };

    Row::new()
        .push(
            Button::new(mainnet_text)
                .padding(iced::Padding::from([4.0, 12.0]))
                .on_press(on_select(NetworkId::Mainnet)),
        )
        .push(
            Button::new(testnet_text)
                .padding(iced::Padding::from([4.0, 12.0]))
                .on_press(on_select(NetworkId::Testnet)),
        )
        .spacing(8)
        .into()
}

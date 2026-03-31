// logic/network.rs
// =========================================
// Network management with auto-switching based on account suffix
// =========================================
use dioxus::prelude::*;
// =========================================

/// Storage key for network ID in localStorage
pub const NETWORK_STORAGE_KEY: &str = "network_id";

/// Represents the network type
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum NetworkId {
    #[default]
    Mainnet,
    Testnet,
}

impl NetworkId {
    /// Get the network ID as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            NetworkId::Mainnet => "mainnet",
            NetworkId::Testnet => "testnet",
        }
    }

    /// Parse from string
    pub fn from_str(value: &str) -> Self {
        if value.trim().to_lowercase() == "testnet" {
            NetworkId::Testnet
        } else {
            NetworkId::Mainnet
        }
    }

    /// Get API base URL for this network
    pub fn api_base_url(&self) -> &'static str {
        match self {
            NetworkId::Mainnet => "https://tx.main.fastnear.com",
            NetworkId::Testnet => "https://tx.test.fastnear.com",
        }
    }

    /// Get the other network URL for cross-network redirects
    pub fn other_network_url(&self) -> &'static str {
        match self {
            NetworkId::Mainnet => "https://testnet.nearrocks.com",
            NetworkId::Testnet => "https://nearrocks.com",
        }
    }

    /// Detect network from account ID suffix
    /// - .testnet → Testnet
    /// - .near, .tg, or anything else → Mainnet
    pub fn from_account_id(account_id: &str) -> Self {
        if account_id.ends_with(".testnet") {
            NetworkId::Testnet
        } else {
            NetworkId::Mainnet
        }
    }

    /// Check if an account ID should trigger network switch
    pub fn should_switch_network(&self, account_id: &str) -> bool {
        let detected = Self::from_account_id(account_id);
        detected != *self
    }
}

impl std::fmt::Display for NetworkId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Get the stored network ID from localStorage
pub fn get_stored_network_id() -> NetworkId {
    get_web_storage()
        .and_then(|storage| storage.get_item(NETWORK_STORAGE_KEY).ok())
        .flatten()
        .map(|value| NetworkId::from_str(&value))
        .unwrap_or(NetworkId::Mainnet)
}

/// Save the network ID to localStorage
pub fn save_network_id(network_id: NetworkId) -> Result<(), wasm_bindgen::JsValue> {
    if let Some(storage) = get_web_storage() {
        storage.set_item(NETWORK_STORAGE_KEY, network_id.as_str())?;
    }
    Ok(())
}

/// Get the web localStorage
fn get_web_storage() -> Option<web_sys::Storage> {
    web_sys::window()
        .and_then(|window| window.local_storage().ok())
        .flatten()
}

/// Initialize network state from localStorage
///
/// This should be used in a `use_effect` hook to load the stored network ID
pub fn use_network_state() -> Signal<NetworkId> {
    let mut network_id = use_signal(|| NetworkId::Mainnet);

    use_effect(move || {
        network_id.set(get_stored_network_id());
    });

    network_id
}

/// Toggle between mainnet and testnet
pub fn toggle_network(current: NetworkId) -> NetworkId {
    match current {
        NetworkId::Mainnet => NetworkId::Testnet,
        NetworkId::Testnet => NetworkId::Mainnet,
    }
}

/// Switch to network based on account ID
/// Returns the new network ID and whether a switch occurred
pub fn switch_network_for_account(current: NetworkId, account_id: &str) -> (NetworkId, bool) {
    let detected = NetworkId::from_account_id(account_id);
    let switched = detected != current;
    if switched {
        let _ = save_network_id(detected);
    }
    (detected, switched)
}
// =========================================
// copyright 2026 by sleet.near

// utils_iced/network.rs
// =========================================
// Network ID enum for iced desktop (no web deps)
// =========================================

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

    /// Get API base URL for this network
    pub fn api_base_url(&self) -> &'static str {
        match self {
            NetworkId::Mainnet => "https://tx.main.fastnear.com",
            NetworkId::Testnet => "https://tx.test.fastnear.com",
        }
    }

    /// Toggle to the other network
    pub fn toggle(self) -> Self {
        match self {
            NetworkId::Mainnet => NetworkId::Testnet,
            NetworkId::Testnet => NetworkId::Mainnet,
        }
    }

    /// Get the other network
    pub fn other(self) -> Self {
        self.toggle()
    }
}

impl std::fmt::Display for NetworkId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
// =========================================

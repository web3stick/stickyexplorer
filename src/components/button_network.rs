// components/button_network.rs
// =========================================
// Network toggle button component
// =========================================
use crate::ui_utils::network::{get_stored_network_id, save_network_id, toggle_network, NetworkId};
use dioxus::prelude::*;
use web_sys::console;
// =========================================

#[component]
pub fn button_network() -> Element {
    let mut network_id = use_signal(|| get_stored_network_id());

    use_effect(move || {
        network_id.set(get_stored_network_id());
    });

    let toggle_network_handler = move |_| {
        let new_network = toggle_network(network_id());
        network_id.set(new_network);

        if let Err(e) = save_network_id(new_network) {
            console::error_1(&e);
        }

        console::log_1(&format!("{}", new_network.as_str()).into());
    };

    let button_class = match network_id() {
        NetworkId::Mainnet => "bg-[#8CA2F5] hover:bg-[#7a91e8] text-white",
        NetworkId::Testnet => "bg-[#C9A8F4] hover:bg-[#b895e3] text-white",
    };

    rsx! {
        button {
            onclick: toggle_network_handler,
            class: "font-medium text-sm transition-colors {button_class}",
            "{network_id().as_str().to_uppercase()}"
        }
    }
}
// =========================================
// copyright 2026 by sleet.near

use crate::logic::network::{
    get_stored_network_id, save_network_id, toggle_network,
};
use dioxus::prelude::*;
use web_sys::console;
// ===========================================
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

    rsx! {
        button {
            onclick: toggle_network_handler,
            "{network_id().as_str().to_uppercase()}"
        }
    }
}
// ===========================================

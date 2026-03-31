// pages/navbar.rs
// =========================================
// Shared navbar component with search and navigation
// =========================================
use dioxus::prelude::*;
use crate::pages::route::Route;
use crate::components::search_bar::search_bar;
use crate::components::button_network::button_network;
// =========================================

/// Shared navbar component.
#[component]
pub fn Navbar() -> Element {
    rsx! {
        header {
            id: "header",
            class: "max-w-7xl mx-auto mb-6",
            div {
                class: "flex flex-col sm:flex-row items-center gap-4 py-4",
                // Logo
                Link {
                    to: Route::Home {},
                    class: "text-xl font-bold hover:opacity-80",
                    "StickyExplorer"
                }
                
                // Search bar (flex-1 to take available space)
                div { class: "flex-1 w-full sm:max-w-md",
                    search_bar {}
                }
                
                // Network button
                div {
                    button_network {}
                }
                
                // Navigation links
                nav {
                    id: "navbar",
                    class: "flex gap-4",
                    Link {
                        to: Route::Home {},
                        class: "hover:text-[#8CA2F5] transition-colors",
                        "Home"
                    }
                }
            }
        }

        main {
            class: "max-w-7xl mx-auto",
            Outlet::<Route> {}
        }
    }
}
// =========================================
// copyright 2026 by sleet.near
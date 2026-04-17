// src/bin/stickyexplorer_iced_main.rs
// =========================================
// Thin binary wrapper to launch the Iced desktop UI
// =========================================
//
// Build and run with:
//   cargo build --features iced_desktop --bin stickyexplorer_iced_main
//   cargo run  --features iced_desktop --bin stickyexplorer_iced_main
//
// Note: This binary uses the existing API client, types, and utility functions
// from the stickyexplorer crate (src/utils/, src/api/, etc.) without any
// web/browser dependencies — pure desktop networking via reqwest + tokio.
// =========================================

use stickyexplorer::iced_pages::app::{AppState, Message};
use iced::{application, Element, Settings, Theme};

fn main() -> iced::Result {
    println!("Starting StickyExplorer (Iced Desktop)...");

    let settings = Settings::default();

    // Use a generic view function that works with any lifetime
    fn view<'a>(state: &'a AppState) -> Element<'a, Message> {
        state.view()
    }

    // Use a mutable update closure
    fn update(state: &mut AppState, msg: Message) -> iced::Task<Message> {
        state.update(msg)
    }

    application(AppState::new, update, view)
        .theme(Theme::Dark)
        .settings(settings)
        .run()
}

use dioxus::prelude::*;
// =========================================
use crate::pages::navbar::Navbar;
use crate::pages::page_home::Home;
use crate::pages::page_blog::Blog;
// =========================================
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}
// =========================================

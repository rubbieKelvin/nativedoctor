//! Root route and app shell.

use dioxus::prelude::*;

use crate::components::Navbar;
use crate::views::Home;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
}

#[component]
pub fn App() -> Element {
    rsx! {
        link { rel: "stylesheet", href: "/assets/styling/main.css" }
        link { rel: "stylesheet", href: "/assets/tailwind.css" }
        Router::<Route> {}
    }
}

//! Top layout: title and outlet.

use dioxus::prelude::*;

use crate::app::Route;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        link { rel: "stylesheet", href: "/assets/styling/navbar.css" }
        div { id: "navbar",
            Link { to: Route::Home {}, "nativedoctor" }
        }
        Outlet::<Route> {}
    }
}

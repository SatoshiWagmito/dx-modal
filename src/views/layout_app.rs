use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::Route;

#[inline_props]
pub fn LayoutApp(cx: Scope) -> Element {
    render! {
        link {
            href: "https://unpkg.com/tailwindcss@2.2.19/dist/tailwind.min.css",
            rel: "stylesheet",
            r#type: "text/css",
        }

        div {
            class: "sticky top-0 left-0 right-0 z-50",
            "[navigation goes here]"
        }

        Outlet::<Route> {}
    }
}

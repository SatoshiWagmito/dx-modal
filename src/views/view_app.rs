use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus_router::hooks::use_navigator;
use crate::Route;


pub fn ViewApp(cx: Scope) -> Element {
    let nav = use_navigator(cx);

    render!(
        h2 {
            class: "text-2xl font-bold leading-7 text-white sm:truncate sm:text-3xl sm:tracking-tight",
            "Modal Example"
        }

        button {
            class: "rounded-md bg-white px-3.5 py-2.5 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50",
            r#type: "button",
            onclick: move |event| {
                nav.push(Route::ViewForm {});
            },
            "Open Modal"
        }

    )
}
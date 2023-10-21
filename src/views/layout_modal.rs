use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus_router::hooks::use_navigator;

use crate::Route;

#[inline_props]
pub fn LayoutModal (cx: Scope) -> Element {
    let nav = use_navigator(cx);

    cx.render(rsx! {
        div {
            id: "popup-modal",
            tabindex: "-1",
            //aria_modal: "true",
            role: "dialog",
            class: "fixed top-0 left-0 right-0 z-50 w-full p-4 overflow-x-hidden overflow-y-auto md:inset-0 h-[calc(100%-1rem)] max-h-full justify-center items-center flex",

            div {
                class: "relative w-full max-w-md h-full",
                div {
                    class: "relative top-0 left-0 right-0 bottom-0 rounded-lg shadow bg-gray-700 bg-opacity-90",
                    button {
                        //data_modal_hide: "popup-modal",
                        onclick: move |event| {
                            nav.go_back();
                        },
                        r#type: "button",
                        class: "absolute top-3 right-2.5 text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm w-8 h-8 ml-auto inline-flex justify-center items-center dark:hover:bg-gray-600 dark:hover:text-white",
                        svg {
                            fill: "none",
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 14 14",
                            //aria_hidden: "true",
                            class: "w-3 h-3",
                            path {
                                stroke_linejoin: "round",
                                stroke_linecap: "round",
                                stroke_width: "2",
                                d: "m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6",
                                stroke: "currentColor"
                            }
                        }
                        span {
                            class: "sr-only",
                            "Close modal"
                        }
                    }
                    Outlet::<Route> { }
                }
            }
        }
        div {
            class: "bg-gray-900 bg-opacity-80 fixed inset-0 z-40"
        }
    })
}

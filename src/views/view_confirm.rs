use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus_router::hooks::use_navigator;

pub fn ViewConfirm(cx: Scope) -> Element {
    let nav = use_navigator(cx);

    render!(
        h3 {
            class: "text-xl font-medium flex items-center justify-between p-5 border-b rounded-t dark:border-gray-600",
            "Agreement"
        }

        div {
            class: "p-6 space-y-6",
            p {
                class: "text-base leading-relaxed text-gray-500 dark:text-gray-400",
                "[paragraph goes here]"
            }
            p {
                class: "text-base leading-relaxed text-gray-500 dark:text-gray-400",
                "[paragraph goes here]"
            }
        }

        div {
            class: "flex items-center p-6 space-x-2 border-t border-gray-200 rounded-b dark:border-gray-600",

            form {
                onsubmit: move |event| {
                    // doesn't work rn
                    let uuid = event.data.values.get("uuid").unwrap().join("");
                    log::info!("Submitted! {:?}", event);
                    todo!();
                },

                input {
                    id: "uuid",
                    name: "uuid",
                    //r#type: "hidden",
                    value: "a6ccd486-e03d-4beb-95c2-96f3741b0b93",
                },

                button {
                    r#type: "submit",
                    class: "text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800",
                    "I accept"
                }
            }

            button {
                onclick: move |event| {
                    nav.go_back();
                },
                class: "text-gray-500 bg-white hover:bg-gray-100 focus:ring-4 focus:outline-none focus:ring-gray-200 rounded-lg border border-gray-200 text-sm font-medium px-5 py-2.5 hover:text-gray-900 focus:z-10 dark:bg-gray-700 dark:text-gray-300 dark:border-gray-500 dark:hover:text-white dark:hover:bg-gray-600 dark:focus:ring-gray-600",
                r#type: "button",
                "Decline"
            }
        }
    )
}
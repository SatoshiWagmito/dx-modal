use dioxus::prelude::*;
use dioxus_router::prelude::*;


pub fn ViewForm(cx: Scope) -> Element {
    render!(
        div {
            class: "px-6 py-6 lg:px-8",
            h3 {
                class: "mb-4 text-xl font-medium text-gray-900 dark:text-white",
                "Sign in to our platform"
            }
            form {
                class: "space-y-6",
                action: "#",div {
                    label {
                        class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                        r#for: "email","Your email"
                    }
                    input {
                        class: "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-600 dark:border-gray-500 dark:placeholder-gray-400 dark:text-white",
                        id: "email",
                        name: "email",
                        placeholder: "name@company.com",
                        required: "",
                        r#type: "email",
                    }
                }
                div {
                    label {
                        class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                        r#for: "password","Your password"
                    }
                    input {
                        class: "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-600 dark:border-gray-500 dark:placeholder-gray-400 dark:text-white",
                        id: "password",
                        name: "password",
                        placeholder: "••••••••",
                        required: "",
                        r#type: "password",
                    }
                }
                div {
                    class: "flex justify-between",
                    div {
                        class: "flex items-start",
                        div {
                            class: "flex items-center h-5",
                            input {
                                class: "w-4 h-4 border border-gray-300 rounded bg-gray-50 focus:ring-3 focus:ring-blue-300 dark:bg-gray-600 dark:border-gray-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:focus:ring-offset-gray-800",
                                id: "remember",
                                required: "",
                                r#type: "checkbox",
                                value: "",
                            }
                        }
                        label {
                            class: "ml-2 text-sm font-medium text-gray-900 dark:text-gray-300",
                            r#for: "remember","Remember me"
                        }
                    }
                    a {
                        class: "text-sm text-blue-700 hover:underline dark:text-blue-500",
                        href: "#","Lost Password?"
                    }
                }
                button {
                    class: "w-full text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800",
                    r#type: "submit","Login to your account"
                }
                div {
                    class: "text-sm font-medium text-gray-500 dark:text-gray-300",
                    "Not registered?"
                    a {
                        class: "text-blue-700 hover:underline dark:text-blue-500",
                        href: "#","Create account"
                    }
                }
            }
        }
    )
}
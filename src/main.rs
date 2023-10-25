#![allow(non_snake_case)]

mod views;

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use wasm_bindgen::prelude::wasm_bindgen;
extern crate wasm_bindgen;

use crate::views::*;



#[rustfmt::skip]
#[derive(Clone, Routable, PartialEq, Eq, Debug)]
enum Route {
    #[layout(LayoutApp)]
        #[route("/")]
        ViewApp {},

        #[nest("/modal")]
            #[layout(LayoutModal)]
                #[route("/form")]
                ViewForm {},

                #[route("/confirm")]
                ViewConfirm {},
            #[end_layout]
        #[end_nest]
    #[end_layout]
    #[route("/:..segments")]
    Err404 { segments: Vec<String> },
}

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    render!(
        Router::<Route> {}
    )
}


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}




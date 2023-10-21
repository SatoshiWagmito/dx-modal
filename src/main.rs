#![allow(non_snake_case)]

mod views;

use dioxus::prelude::*;
use dioxus_router::prelude::*;
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




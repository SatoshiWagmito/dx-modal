use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[inline_props]
pub fn Err404(cx: Scope, segments: Vec<String>) -> Element {

    render!(
        h2 {
            "404"
        },
        div {
            segments.join(",")
        }
    )
}
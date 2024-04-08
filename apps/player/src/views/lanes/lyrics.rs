use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;

#[component]
pub fn LyricsLane() -> Element {
    rsx! {
        div {
            class: class!(h_16),
            "lyrics"
        }
    }
}
use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;

use crate::views::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: class!(w_screen h_screen),
            Video {
            }
            Lyrics {
            }
        }
    }
}
use std::sync::Arc;
use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;

use song_proto::prelude::*;
use crate::views::*;

#[component]
pub fn Bars() -> Element {
    let song = consume_context::<Signal<Arc<Tab>>>();
    rsx! {
        div {
            class: class!(flex flex_wrap m_8),
            for bar in song().bars.clone() {
                Bar {
                    bar, 
                }
            }
        }
    }
}

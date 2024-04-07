use std::sync::Arc;
use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;

use song_proto::prelude::*;
use crate::views::*;

#[component]
pub fn Section() -> Element {
    let song = consume_context::<Signal<Arc<Tab>>>();
    let bars = song().bars.clone();
    rsx! {
        div {
            class: class!(flex flex_wrap m_8),
            for (index, bar) in bars.iter().enumerate() {
                if bar.props.section_index == 1 || true {
                    Bar {
                        index, 
                    }
                }
            }
        }
    }
}

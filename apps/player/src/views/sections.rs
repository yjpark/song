use std::sync::Arc;
use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;

use song_proto::prelude::*;

#[component]
pub fn Sections() -> Element {
    let song = consume_context::<Signal<Arc<Tab>>>();
    rsx! {
        ul {
            class: class!(timeline),
            for section in song().form.sections.clone() {
                li {
                    div {
                        class: class!(timeline_start),
                        "{section.id}",
                    }
                    div {
                        class: class!(timeline_middle),
                    }
                    div {
                        class: class!(timeline_end timeline_box),
                        "{section.kind}",
                    }
                }
            }
        }
    }
}
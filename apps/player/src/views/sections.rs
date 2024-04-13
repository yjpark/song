use std::sync::Arc;
use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fi_icons;

use song_proto::prelude::*;

#[component]
pub fn Sections() -> Element {
    let song = consume_context::<Signal<Arc<Tab>>>();
    let sections = song().form.sections.clone();
    rsx! {
        div {
            ul {
                class: class!(timeline justify_center),
                for (index, section) in sections.iter().enumerate() {
                    li {
                        if index > 0 {
                            hr {}
                        }
                        div {
                            class: class!(timeline_start),
                            "{section.id}",
                        }
                        div {
                            class: class!(timeline_middle),
                            Icon {
                                icon: fi_icons::FiDisc,
                            }
                        }
                        div {
                            class: class!(timeline_end timeline_box),
                            "{section.kind}",
                        }
                        if index < sections.len() - 1 {
                            hr {}
                        }
                    }
                }
            }
        }
    }
}
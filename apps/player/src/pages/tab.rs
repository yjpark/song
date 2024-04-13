use std::ops::Deref;

use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;
use song_proto::tab::LoadTabResult;

use crate::app::model;
use crate::views::*;

#[component]
pub fn Tab() -> Element {
    match model::TAB.read().deref() {
        Ok(song) => {
            use_context_provider(|| Signal::new(song.clone()));
            rsx! {
                div {
                    class: class!(w_screen h_screen),
                    Sections {
                    },
                    Bars {
                    }
                }
            }
        },
        Err(error) => rsx! {
            div {
                class: class!(w_screen h_screen flex),
                "ERR",
            }
        }
    }
}
use std::sync::Arc;

use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;

use song_proto::prelude::*;

use crate::views::entries::*;

#[component]
pub fn ChordLane(lane: ReadOnlySignal<Option<Arc<BarLane>>>) -> Element {
    let Some(lane) = lane() else {
        return None;
    };
    rsx! {
        for entry in lane.entries {

        }
        div {
            class: class!(h_16),
            "chord"
        }
    }
}
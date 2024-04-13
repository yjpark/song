use std::sync::Arc;

use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;

use song_proto::prelude::*;

use crate::views::entries::*;

#[component]
pub fn BarLane(lane: ReadOnlySignal<Option<Arc<BarLane>>>) -> Element {
    let Some(lane) = lane() else {
        return None;
    };
    rsx! {
        div {
            class: class!(flex flex_row),
            for entry in lane.entries.clone() {
                Entry {
                    entry,
                }
            }
        }
    }
}
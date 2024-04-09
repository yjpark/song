use std::sync::Arc;
use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;

use song_proto::prelude::*;
use crate::views::BarLane;

#[component]
pub fn Bar(bar: ReadOnlySignal<Arc<TabBar>>) -> Element {
    rsx! {
        div {
            class: class!(flex flex_col w_64 my_16),
            BarLane {
                lane: bar().get_lane_of_kind(LaneKind::Chord, None),
            }
            BarLane {
                lane: bar().get_lane_of_kind(LaneKind::Lyrics, None),
            }
        }
    }
}

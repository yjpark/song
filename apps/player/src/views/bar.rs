use std::sync::Arc;
use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;

use song_proto::prelude::*;
use crate::views::lanes::*;

#[component]
pub fn Bar(bar: ReadOnlySignal<Arc<TabBar>>) -> Element {
    rsx! {
        div {
            class: class!(flex flex_col),
            ChordLane {
                lane: bar().get_lane_of_kind(LaneKind::Chord, None),
            }
            LyricsLane {

            }
            div {
                //{bar.props.bar_index.to_string()}
                class: class!(w_64),
                key: "{index}",
                "{bar}"
            }
        }
    }
}

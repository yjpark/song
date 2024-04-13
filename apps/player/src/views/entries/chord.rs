use std::sync::Arc;

use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;

use song_proto::prelude::*;

#[component]
pub fn ChordEntry(entry: ReadOnlySignal<Arc<LaneEntry>>, chord: ReadOnlySignal<Chord>) -> Element {
    rsx! {
        div {
            "{chord}"
        }
    }
}

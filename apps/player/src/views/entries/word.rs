use std::sync::Arc;

use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;

use song_proto::prelude::*;

use crate::views::entries::*;

#[component]
pub fn WordEntry(entry: ReadOnlySignal<Arc<LaneEntry>>, word: ReadOnlySignal<LyricWord>) -> Element {
    rsx! {
        span {
            "{word}",
        }
    }
}

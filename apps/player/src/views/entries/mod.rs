use std::sync::Arc;

use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;

use song_proto::prelude::*;

pub mod lyric;
pub mod chord;

pub use lyric::LyricEntry;
pub use chord::ChordEntry;

pub fn Entry(entry: ReadOnlySignal<Arc<LaneEntry>>) -> Element {
    match entry().proto() {
        ProtoEntry::Core(core) => rsx! {
            ChordEntry {

            }
        },
        _ => None,
    }
}
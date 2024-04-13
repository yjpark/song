use std::sync::Arc;

use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;

use song_proto::prelude::*;

pub mod word;
pub mod chord;

pub use word::WordEntry;
pub use chord::ChordEntry;

#[component]
pub fn Entry(entry: ReadOnlySignal<Arc<LaneEntry>>) -> Element {
    let entry = entry();
    match entry.proto() {
        ProtoEntry::Core(core) => match core {
            CoreEntry::Chord(chord, _) => rsx!{
                ChordEntry {
                    entry: entry.clone(),
                    chord: chord.clone(),
                }
            },
            _ => None,
        },
        ProtoEntry::Lyric(lyric) => match lyric {
            LyricEntry::Word(word, _) => rsx! {
                WordEntry {
                    entry: entry.clone(),
                    word: word.clone(),
                }
            }
        }
        _ => None,
    }
}
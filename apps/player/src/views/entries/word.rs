use std::sync::Arc;

use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;

use song_proto::prelude::*;

use crate::views::entries::*;

#[component]
pub fn WordEntry(entry: ReadOnlySignal<Arc<LaneEntry>>, word: ReadOnlySignal<LyricWord>) -> Element {
    let class = class!(bg_gray_400 border border_white);
    let units = entry().tied_units();
    let width = units.0 * 400.0;
    let style = format!("width: {}px", width);
    rsx! {
        span {
            class,
            style,
            "{word}",
        }
    }
}

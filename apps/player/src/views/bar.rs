use std::sync::Arc;
use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;

use song_proto::prelude::*;

#[component]
pub fn Bar() -> Element {
    rsx! {
        div {
            //{bar.props.bar_index.to_string()}
            class: class!(w_96),
            "TODO"
        }
    }
}

use std::sync::Arc;
use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;

use song_proto::prelude::*;

#[component]
pub fn Bar(index: usize) -> Element {
    let song = consume_context::<Signal<Arc<Tab>>>();
    let bar = song().bars[index].clone();
    rsx! {
        div {
            //{bar.props.bar_index.to_string()}
            class: class!(w_64),
            key: "{index}",
            "{bar}"
        }
    }
}

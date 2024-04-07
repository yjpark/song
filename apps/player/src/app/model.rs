use dioxus::prelude::*;
use song_proto::prelude::*;

static TAB_BYTES: &'static [u8] = include_bytes!("../../assets/tabs/anne.ron");

pub static TAB: GlobalSignal<Tab> = Signal::global(|
| {
    let tab = load_tab(&TAB_BYTES)
        .or_else(ProtoTab::new_empty());
    Tab::try_parse_arc(tab, false, None)
});
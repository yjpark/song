use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;


#[component]
pub fn Lyrics() -> Element {
    let lyrics = r#"
        こんな僕でも やれることがある
        頑張って ダメで 悩んで
        汗流して できなくって
        バカなやつだって 笑われたって
        涙こらえて
        なんにもないけど
        いつでも 野に咲く花のように
        君が優しかったから 僕は
        元気でいるから
    "#.split("\n");
    rsx! {
        div {
            for line in lyrics {
                div {
                    {line}
                }
            }
        }
    }
}
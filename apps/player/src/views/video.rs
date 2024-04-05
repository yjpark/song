use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;

#[component]
pub fn Video() -> Element {
    let code = r#"
        let msg = await dioxus.recv();
        console.log(msg);
        /*
        var player = videojs("video-player", {
            "techOrder": ["youtube"],
            "sources": [
                {
                    "type": "video/youtube",
                    "src": "https://www.youtube.com/watch?v=EtWnywVnSls"
                }
            ]
        });
         */
    "#;
    let mut eval = eval(code);
    let future = use_resource(move || {
        to_owned![eval];
        async move {
            eval.send("Setup video player".into());
        }
    });
    rsx! {
        video {
            class: class!(card card_body "video-js"),
            id: "video-player",
            controls: true,
            src: "/video/mr.lonely.webm",
            // src: "http://vjs.zencdn.net/v/oceans.mp4",
        }
    }
}
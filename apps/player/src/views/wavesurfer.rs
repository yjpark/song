use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;

#[component]
pub fn WaveSurfer() -> Element {
    let code = r#"
        let msg = await dioxus.recv();
        console.log(msg);
        var wavesurfer = WaveSurfer.create({
            container: document.getElementById("video-waveform"),
            waveColor: "rgb(200, 0, 200)",
            progressColor: "rgb(100, 0, 100)",
            media: document.getElementById("video-player"),
        });
    "#;
    let mut eval = eval(code);
    let future = use_resource(move || {
        to_owned![eval];
        async move {
            eval.send("Setup video waveform".into());
        }
    });
    rsx! {
        div {
            class: class!(w_full),
            div {
                class: class!(w_auto h_16 mx_2),
                id: "video-waveform",
            }
        }
    }
}

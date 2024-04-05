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
        // Update the zoom level on slider change
        wavesurfer.once('decode', () => {
            const slider = document.getElementById("video-zoom")

            slider.addEventListener('input', (e) => {
                const minPxPerSec = e.target.valueAsNumber
                wavesurfer.zoom(minPxPerSec)
            })
        })
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
            class: class!(card),
            input {
                class: class!(card_title),
                id: "video-zoom",
                r#type: "range",
                min: "10",
                max: "1000",
                value: "100",
            }
            div {
                class: class!(card_body),
                id: "video-waveform",
            }
        }
    }
}

mod download_form;
mod download_playlist;
mod download_radio;

use dioxus::prelude::*;

use crate::{
    app::{download_playlist::PlaylistDownloadForm, download_radio::RadioDownloadForm},
    logic::download::CommonOpts,
};

mod macros {
    macro_rules! write_log {
        ($state:expr, $($arg:tt)*) => {{
            let s = format!($($arg)*);
            println!("{}", s);
            $state.with_mut(|state| state.logs.push(s));
        }};
    }

    pub(crate) use write_log;
}

macro_rules! option_checkbox {
    ($label:expr, $state:expr, $field:expr) => {
        rsx!(label {
            input {
                r#type: "checkbox",
                checked: $state.read().opts.$field,
                oninput: move |e| {
                    if let Ok(v) = e.value.parse::<bool>() {
                        $state.with_mut(|state| state.opts.$field = v);
                    }
                }
            }
            $label
        })
    };
}

struct CommonState {
    downloading: bool,
    opts: CommonOpts,
    logs: Vec<String>,
}

#[allow(clippy::redundant_closure_call)]
pub fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || CommonState {
        downloading: false,
        opts: CommonOpts {
            overwrite: false,
            set_track_number: false,
            write_youtube_id: true,
            exclude_video: true,
        },
        logs: Vec::new(),
    });
    let common_state = use_shared_state::<CommonState>(cx).unwrap();

    cx.render(rsx! {
        fieldset { class: "w-full flex flex-col gap-2 p-1", "disabled": common_state.read().downloading,
            h1 { class: "text-2xl", "Youtube Music Downloader" }
            RadioDownloadForm {}
            PlaylistDownloadForm {}
            option_checkbox!("Overwrite existing files", common_state, overwrite),
            option_checkbox!("Set track number", common_state, set_track_number),
            option_checkbox!("Write youtube id", common_state, write_youtube_id),
            option_checkbox!(
                "Exclude video (if counterpart audio exists, it will be downloaded)", 
                common_state, 
                exclude_video
            ),
            div { class: "border border-red-400 rounded-md p-2 flex flex-col gap-2",
                h3 { class: "text-xl", "Logs" }
                button {
                    class: "bg-gray-200 hover:bg-gray-300 rounded-md p-2",
                    onclick: move |_| {
                        common_state.with_mut(|state| state.logs.clear());
                    },
                    "Clear logs"
                }
                pre { class: "text-white bg-black font-mono p-1 overflow-auto",
                    code {
                        common_state.read().logs.iter().rev().map(|log| rsx!(
                            div {
                                "{log}"
                            }
                        ))
                    }
                }
            }
        }
    })
}

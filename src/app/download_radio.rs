use anyhow::Result;
use dioxus::prelude::*;

use crate::{
    app::{macros::write_log, CommonState},
    external::ytmusic::queue::get_queue,
    interface::video_id::VideoId,
    logic::download::download_tracks,
};

use super::download_form::DownloadForm;

#[allow(clippy::redundant_closure_call)]
#[component]
pub fn RadioDownloadForm(cx: Scope) -> Element {
    let common_state = use_shared_state::<CommonState>(cx).unwrap();

    let download_single_song = use_state(cx, || false);

    let download = move |url: String| {
        common_state.with_mut(|state| {
            state.downloading = true;
        });
        cx.spawn({
            to_owned![common_state, download_single_song];
            async move {
                let res = async {
                    let video_id = VideoId::from_amibgous_url(&url);
                    write_log!(common_state, "Fetching radio: {}", url);
                    let radio = get_queue(&video_id, !*download_single_song.get()).await?;
                    let tracks_len = radio.tracks.len();
                    write_log!(
                        common_state,
                        "Download starting. Track list:\n{} \n({} tracks)",
                        radio
                            .tracks
                            .iter()
                            .map(|track| format!("  {}\n", track.title))
                            .collect::<Vec<String>>()
                            .join(""),
                        tracks_len
                    );
                    let opts = { common_state.read().opts };
                    download_tracks(radio.tracks, opts, |track, res| match res {
                        Ok(_) => {
                            write_log!(common_state, "Downloaded: \"{}\"", track.title)
                        }
                        Err(e) => {
                            write_log!(common_state, "Error downloading \"{}\": {}", track.title, e)
                        }
                    })
                    .await;
                    write_log!(common_state, "Download completed!");
                    Ok(()) as Result<()>
                }
                .await;
                if let Err(e) = &res {
                    write_log!(common_state, "Error fetching info: {}", e);
                }
                common_state.with_mut(|state| {
                    state.downloading = false;
                });
            }
        })
    };

    cx.render(rsx! {
        div { class: "p-2 border border-black rounded-md flex flex-col gap-2",
            DownloadForm { title: "Download radio of track", onclick: download }
            label {
                input {
                    r#type: "checkbox",
                    checked: "{download_single_song}",
                    oninput: move |e| {
                        if let Ok(v) = e.value.parse::<bool>() {
                            download_single_song.set(v);
                        }
                    }
                }
                "Download just one song instead of radio"
            }
        }
    })
}

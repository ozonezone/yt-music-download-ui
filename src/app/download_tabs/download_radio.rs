use anyhow::Result;
use dioxus::prelude::*;
use openapi::{
    apis::default_api::get_queue,
    models::{QueueGetParams, QueueTrack},
};

use crate::{
    app::{macros::write_log, CommonState},
    config::API_CONFIG,
    interface::video_id::VideoId,
    logic::download::download_tracks,
};

use super::download_form::DownloadForm;

/// Remove (or replace to counterpart if tracks has) track from tracks
fn process_invalid_tracks(
    tracks: Vec<QueueTrack>,
    common_state: UseSharedState<CommonState>,
) -> Vec<QueueTrack> {
    tracks
        .into_iter()
        .filter_map(|track| {
            let Some(video_type) = track.video_type else {
                write_log!(
                    common_state,
                    "Removed video track. Failed to find video_type!: \"{}\"",
                    track.title
                );
                return None;
            };
            if !video_type.is_music() {
                if let Some(counterpart) = track.counterpart {
                    let Some(video_type) = counterpart.video_type else {
                        write_log!(
                            common_state,
                            "Removed video track. Failed to find video_type!: \"{}\"",
                            track.title
                        );
                        return None;
                    };
                    if video_type.is_music() {
                        write_log!(
                            common_state,
                            "Replaced video track with counterpart: \"{}\"",
                            track.title
                        );
                        return Some(*counterpart);
                    }
                }
                write_log!(common_state, "Removed video track: \"{}\"", track.title);
                None
            } else {
                Some(track)
            }
        })
        .collect()
}

#[allow(clippy::redundant_closure_call)]
#[component]
pub fn RadioDownloadForm(cx: Scope) -> Element {
    let common_state = use_shared_state::<CommonState>(cx).unwrap();

    let download_single_song = use_state(cx, || false);

    let download = move |input: String| {
        common_state.with_mut(|state| {
            state.downloading = true;
        });
        cx.spawn({
            to_owned![common_state, download_single_song];
            async move {
                let res = async {
                    let video_id = VideoId::from_id_or_url(&input);
                    write_log!(common_state, "Fetching radio: {}", input);
                    let mut radio = get_queue(
                        &API_CONFIG,
                        QueueGetParams {
                            video_id: video_id.id,
                            radio: !*download_single_song,
                        },
                    )
                    .await?;

                    if common_state.read().opts.exclude_video {
                        write_log!(common_state, "Removing video tracks");
                        radio.tracks = process_invalid_tracks(radio.tracks, common_state.clone());
                    }

                    let opts = { common_state.read().opts };
                    download_tracks(
                        radio.tracks.into_iter().map(|t| t.into()).collect(),
                        opts,
                        |msg| write_log!(common_state, "{msg}"),
                    )
                    .await;
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

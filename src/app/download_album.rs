use anyhow::Result;
use dioxus::prelude::*;

use crate::{
    app::{macros::write_log, CommonState},
    external::ytmusic::playlist::get_playlist,
    interface::playlist_id::PlaylistId,
    logic::download::download_tracks,
};

use super::download_form::DownloadForm;

#[allow(clippy::redundant_closure_call)]
#[component]
pub fn AlbumDownloadForm(cx: Scope) -> Element {
    let common_state = use_shared_state::<CommonState>(cx).unwrap();

    let download = move |url: String| {
        common_state.with_mut(|state| {
            state.downloading = true;
        });
        cx.spawn({
            to_owned![common_state];
            async move {
                let res = async {
                    let playlist_id = PlaylistId::from_ambigous_url(&url);
                    write_log!(common_state, "Fetching playlist: {}", url);
                    let mut playlist = get_playlist(&playlist_id).await?;

                    if common_state.read().opts.exclude_video {
                        write_log!(common_state, "Removing video tracks");
                        playlist.tracks.retain(|track| {
                            if track.video_type.is_music() {
                                true
                            } else {
                                write_log!(
                                    common_state,
                                    "Removed video track: \"{}\"",
                                    track.title
                                );
                                false
                            }
                        });
                    }

                    let tracks_len = playlist.tracks.len();
                    write_log!(
                        common_state,
                        "Download starting. Track list:\n{}({} tracks)",
                        playlist
                            .tracks
                            .iter()
                            .map(|track| format!("  {}\n", track.title))
                            .collect::<Vec<String>>()
                            .join(""),
                        tracks_len
                    );
                    let opts = { common_state.read().opts };
                    download_tracks(playlist.tracks, opts, |track, res| match res {
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
            DownloadForm { title: "Download playlist", onclick: download }
        }
    })
}

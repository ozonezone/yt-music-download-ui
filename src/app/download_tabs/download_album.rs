use anyhow::Result;
use dioxus::prelude::*;
use openapi::{
    apis::default_api::get_album_by_playlist_id,
    models::{Album, AlbumGetByPlaylistIdParams},
};

use crate::{
    app::{macros::write_log, CommonState},
    config::API_CONFIG,
    interface::{
        browse_id::{self, BrowseId},
        playlist_id::PlaylistId,
        video_id::VideoId,
    },
    logic::download::download_tracks,
};

use super::download_form::DownloadForm;

async fn get_album(ambiguous_str: String) -> Result<Album> {
    if let Ok(playlist_id) = PlaylistId::from_url(&ambiguous_str) {
        return get_album_by_playlist_id(
            &API_CONFIG,
            AlbumGetByPlaylistIdParams {
                playlist_id: playlist_id.id,
            },
        )
        .await;
    } else if Ok(browse_id) = BrowseId::from_url(&ambiguous_str) {
        return get_album(browse_id.id).await;
    } else {
        get_album_by_playlist_id(
            &API_CONFIG,
            AlbumGetByPlaylistIdParams {
                playlist_id: ambiguous_str,
            },
        )
        .await
    }
}

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
                    write_log!(common_state, "Fetching album: {}", url);
                    let mut album = get_playlist(&playlist_id).await?;

                    if common_state.read().opts.exclude_video {
                        write_log!(common_state, "Removing video tracks");
                        album.tracks.retain(|track| {
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

                    let tracks_len = album.tracks.len();
                    write_log!(
                        common_state,
                        "Download starting. Track list:\n{}({} tracks)",
                        album
                            .tracks
                            .iter()
                            .map(|track| format!("  {}\n", track.title))
                            .collect::<Vec<String>>()
                            .join(""),
                        tracks_len
                    );
                    let opts = { common_state.read().opts };
                    download_tracks(album.tracks, opts, |track, res| match res {
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
            DownloadForm { title: "Download album", onclick: download }
        }
    })
}

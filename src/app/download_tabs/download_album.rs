use anyhow::Result;
use dioxus::prelude::*;
use openapi::{
    apis::default_api::{get_album, get_album_by_playlist_id},
    models::{AlbumGetByPlaylistIdParams, AlbumGetParams, AlbumResult},
};

use crate::{
    app::{macros::write_log, CommonState},
    config::API_CONFIG,
    external::ytmusic::CommonTrack,
    interface::{browse_id::BrowseId, playlist_id::PlaylistId},
    logic::download::download_tracks,
};

use super::download_form::DownloadForm;

async fn fetch_album(ambiguous_str: String) -> Result<AlbumResult> {
    let album = if let Ok(playlist_id) = PlaylistId::from_url(&ambiguous_str) {
        get_album_by_playlist_id(
            &API_CONFIG,
            AlbumGetByPlaylistIdParams {
                playlist_id: playlist_id.id,
            },
        )
        .await?
    } else if let Ok(browse_id) = BrowseId::from_url(&ambiguous_str) {
        get_album(
            &API_CONFIG,
            AlbumGetParams {
                browse_id: browse_id.id,
            },
        )
        .await?
    } else {
        get_album_by_playlist_id(
            &API_CONFIG,
            AlbumGetByPlaylistIdParams {
                playlist_id: ambiguous_str,
            },
        )
        .await?
    };
    Ok(album)
}

#[allow(clippy::redundant_closure_call)]
#[component]
pub fn AlbumDownloadForm(cx: Scope) -> Element {
    let common_state = use_shared_state::<CommonState>(cx).unwrap();

    let download = move |input: String| {
        common_state.with_mut(|state| {
            state.downloading = true;
        });
        cx.spawn({
            to_owned![common_state];
            async move {
                let res = async {
                    write_log!(common_state, "Fetching album: {}", input);
                    let mut album = fetch_album(input).await?;

                    if common_state.read().opts.exclude_video {
                        write_log!(common_state, "Removing video tracks");
                        album.tracks.retain(|track| {
                            if let Some(video_type) = &track.video_type {
                                if video_type.is_music() {
                                    return true;
                                }
                                write_log!(
                                    common_state,
                                    "Removed video track: \"{}\"",
                                    track.title
                                );
                                false
                            } else {
                                write_log!(
                                    common_state,
                                    "[warn] Failed to find video_type but not removed because this can be null with private album: \"{}\"",
                                    track.title
                                );
                                true
                            }
                        });
                    }

                    let mut tracks: Vec<CommonTrack> =
                        album.tracks.into_iter().flat_map(|track| track.try_into()).collect();
                    // Use album info for tracks
                    tracks.iter_mut().for_each(|track| {
                        track.thumbnails = album.thumbnails.clone();
                        track.album = Some(album.title.clone());
                        track.artists = album.artists.clone().unwrap_or_default();
                    });

                    let mut opts = { common_state.read().opts };
                    // Always set track number for albums
                    opts.set_track_number = true;
                    download_tracks(tracks, opts, |msg| write_log!(common_state, "{msg}")).await;
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

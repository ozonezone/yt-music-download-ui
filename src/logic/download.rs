use anyhow::Context;
use futures::StreamExt;
use serde::Deserialize;

use crate::{constants::AUTH_FILE, external::ytmusic::interface::CommonTrack};

use super::download_track::download_track;

#[derive(Clone, Copy)]
pub(crate) struct CommonOpts {
    pub overwrite: bool,
    pub set_track_number: bool,
    pub write_youtube_id: bool,
}

#[derive(Deserialize)]
pub(crate) struct Auth {
    pub token: AuthToken,
}
#[derive(Deserialize)]
pub(crate) struct AuthToken {
    pub access_token: String,
}

pub(crate) async fn download_tracks(
    tracks: Vec<impl Into<CommonTrack>>,
    opts: CommonOpts,
    callback: impl Fn(CommonTrack, anyhow::Result<()>) + Copy,
) {
    let mut futures = Vec::new();
    let len = tracks.len();
    for (idx, track) in tracks.into_iter().enumerate() {
        let track: CommonTrack = track.into();
        let track_number: Option<(u16, u16)> = if opts.set_track_number {
            Some(((idx + 1) as u16, len as u16))
        } else {
            None
        };
        futures.push(async move {
            let result = async {
                let auth_str = tokio::fs::read_to_string(&AUTH_FILE)
                    .await
                    .context("Failed to load auth file")?;
                let auth: Auth = serde_json::from_str(&auth_str)?;
                let result: anyhow::Result<()> = download_track(
                    &track,
                    opts.overwrite,
                    track_number,
                    opts.write_youtube_id,
                    &Some(auth.token.access_token),
                )
                .await;
                result
            }
            .await;
            callback(track, result);
        });
    }

    let stream = futures::stream::iter(futures).buffer_unordered(10);

    stream.collect::<Vec<_>>().await;
}

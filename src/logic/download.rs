use std::path::PathBuf;

use anyhow::Context;
use futures::StreamExt;
use serde::Deserialize;

use crate::{config::CONFIG, external::ytmusic::CommonTrack};

use super::download_track::{download_track, DownloadOpts};

#[derive(Clone, Copy)]
pub(crate) struct CommonOpts {
    pub overwrite: bool,
    pub set_track_number: bool,
    pub write_youtube_id: bool,
    pub exclude_video: bool,
}

#[derive(Deserialize)]
pub(crate) struct Auth {
    pub token: AuthToken,
}
#[derive(Deserialize)]
pub(crate) struct AuthToken {
    pub access_token: String,
}

fn sanitize(s: &str) -> String {
    sanitize_filename::sanitize_with_options(
        s,
        sanitize_filename::Options {
            windows: false,
            truncate: false,
            replacement: " ",
        },
    )
}

fn save_path(track: &CommonTrack) -> anyhow::Result<PathBuf> {
    let album = track.album.as_ref().context("No album found")?;
    let artist = track
        .artists
        .iter()
        .map(|x| x.name.clone())
        .collect::<Vec<_>>()
        .join(", ");

    let mut path = PathBuf::from(&CONFIG.download_path);
    path.push(sanitize(&artist));
    path.push(sanitize(album));
    let mut title = sanitize(&track.title);
    title.push_str(".m4a");
    path.push(title);

    Ok(path)
}

pub(crate) async fn download_tracks(
    tracks: Vec<CommonTrack>,
    opts: CommonOpts,
    log: impl Fn(&str) + Copy,
) {
    let len = tracks.len();

    log("Download started!");
    log(&format!(
        "Track list:\n{}\n└─ ({} tracks)",
        tracks
            .iter()
            .map(|track| format!("├─ {}\t({})", track.title, track.video_id))
            .collect::<Vec<String>>()
            .join("\n"),
        len
    ));

    let mut futures = Vec::new();
    for (idx, track) in tracks.into_iter().enumerate() {
        let track_number: Option<(u16, u16)> = if opts.set_track_number {
            Some(((idx + 1) as u16, len as u16))
        } else {
            None
        };
        futures.push(async move {
            let auth_str = tokio::fs::read_to_string(&format!("{}/auth.json", CONFIG.config_path))
                .await
                .context("Failed to load auth file")?;
            let auth: Auth = serde_json::from_str(&auth_str)?;
            let path = save_path(&track)?;
            let result: anyhow::Result<()> = download_track(
                &track,
                &Some(auth.token.access_token),
                &path,
                DownloadOpts {
                    track_number,
                    overwrite: opts.overwrite,
                    write_youtube_id: opts.write_youtube_id,
                },
            )
            .await;

            match result {
                Ok(_) => log(&format!(
                    "Downloaded: [{} ({})] to [{}]]",
                    track.title,
                    track.video_id,
                    path.to_string_lossy()
                )),
                Err(e) => log(&format!(
                    "Failed to download: [{} ({})] to [{}]: {}",
                    track.title,
                    track.video_id,
                    path.to_string_lossy(),
                    e
                )),
            }
            Ok::<(), anyhow::Error>(())
        });
    }

    let stream = futures::stream::iter(futures).buffer_unordered(10);

    stream.collect::<Vec<_>>().await;

    log("All downloads completed!");
}

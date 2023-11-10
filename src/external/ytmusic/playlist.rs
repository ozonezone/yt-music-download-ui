use anyhow::Result;
use serde::Deserialize;
use tokio::process::Command;

use crate::{constants::AUTH_FILE, interface::playlist_id::PlaylistId};

#[derive(Deserialize, Clone, Debug)]
pub(crate) struct MusePlaylist {
    pub tracks: Vec<MusePlaylistItem>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MusePlaylistItem {
    pub video_id: String,
    pub title: String,
    pub artists: Vec<MuseArtistRun>,
    pub album: Option<MuseAlbum>,
    pub thumbnails: Vec<MuseThumbnail>,
    pub video_type: Option<MuseVideoType>,
    pub year: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub(crate) struct MuseArtistRun {
    pub name: String,
    pub id: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub(crate) struct MuseAlbum {
    pub name: String,
    pub id: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub(crate) struct MuseThumbnail {
    pub url: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum MuseVideoType {
    MusicVideoTypeOmv,
    MusicVideoTypeUgc,
    MusicVideoTypeAtv,
}

impl MusePlaylistItem {
    /// Get best quality thumbnail of track
    pub fn extract_best_thumbnail(&self) -> Option<&MuseThumbnail> {
        let mut best_thumbnail: Option<&MuseThumbnail> = None;
        for thumbnail in &self.thumbnails {
            let (best_width, _best_height) = match &best_thumbnail {
                Some(best_thumbnail) => (best_thumbnail.width, best_thumbnail.height),
                None => (0, 0),
            };

            if thumbnail.width > best_width {
                best_thumbnail = Some(thumbnail);
            }
        }
        best_thumbnail
    }
}

pub(crate) async fn get_playlist(playlist: &PlaylistId) -> Result<MusePlaylist> {
    let output = Command::new("deno")
        .args([
            "run",
            "-A",
            "./scripts/api/getPlaylist.ts",
            &playlist.id,
            &AUTH_FILE,
        ])
        .output()
        .await?;
    if output.status.success() {
        let json: MusePlaylist = serde_json::from_str(&String::from_utf8(output.stdout)?)?;
        Ok(json)
    } else {
        Err(anyhow::anyhow!(format!(
            "failed to get playlist info: stdout: {}, stderr: {}",
            &String::from_utf8(output.stdout)?,
            &String::from_utf8(output.stderr)?
        )))
    }
}

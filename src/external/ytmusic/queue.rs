use anyhow::Result;
use tokio::process::Command;

use crate::{constants::AUTH_FILE, interface::video_id::VideoId};

use super::playlist::MusePlaylist;

// #[derive(Deserialize, Clone, Debug)]
// pub(crate) struct MuseQueue {
//     pub id: String,
//     pub title: String,
//     pub tracks: Vec<MusePlaylistItem>,
// }

/// Get radio of song using ytmusicapi
pub(crate) async fn get_queue(video_id: &VideoId, radio: bool) -> Result<MusePlaylist> {
    let output = Command::new("deno")
        .args([
            "run",
            "-A",
            "./scripts/api/getQueue.ts",
            &video_id.id,
            &AUTH_FILE,
            if radio { "true" } else { "false" },
        ])
        .output()
        .await?;
    if output.status.success() {
        let json: MusePlaylist = serde_json::from_str(&String::from_utf8(output.stdout)?)?;
        Ok(json)
    } else {
        Err(anyhow::anyhow!(format!(
            "failed to get radio info: stdout: {}, stderr: {}",
            &String::from_utf8(output.stdout)?,
            &String::from_utf8(output.stderr)?
        )))
    }
}

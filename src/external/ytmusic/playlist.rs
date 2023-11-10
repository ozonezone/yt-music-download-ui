use anyhow::Result;
use tokio::process::Command;

use crate::{constants::AUTH_FILE, interface::playlist_id::PlaylistId};

use super::interface::playlist::Playlist;

pub(crate) async fn get_playlist(playlist: &PlaylistId) -> Result<Playlist> {
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
        let str = String::from_utf8(output.stdout)?;
        let jd = &mut serde_json::Deserializer::from_str(&str);
        let json: Playlist = serde_path_to_error::deserialize(jd)?;
        Ok(json)
    } else {
        Err(anyhow::anyhow!(format!(
            "failed to get playlist info: stdout: {}, stderr: {}",
            &String::from_utf8(output.stdout)?,
            &String::from_utf8(output.stderr)?
        )))
    }
}

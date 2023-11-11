use anyhow::Result;
use serde::Deserialize;
use tokio::process::Command;

use crate::interface::video_id::VideoId;

#[derive(Deserialize, Clone)]
pub(crate) struct YtdlVideoInfo {
    pub formats: Vec<YtdlVideoInfoFormat>,
    // pub album: Option<String>,
    // pub artist: Option<String>,
    // pub track: Option<String>,
}

#[derive(Deserialize, Clone)]
pub(crate) struct YtdlVideoInfoFormat {
    /// 140 is m4a, 128kbps. 141 is m4a, 256kbps which is avilable on only premium account.
    pub format_id: String,
    pub url: String,
    pub acodec: String,
    // pub vcodec: String,
}

impl YtdlVideoInfo {
    /// Get best quality audio of video or audio
    pub fn get_best_audio(&self) -> Option<YtdlVideoInfoFormat> {
        let mut best_audio: Option<YtdlVideoInfoFormat> = None;
        for format in &self.formats {
            if format.acodec == "none" {
                continue;
            }
            /*if format.format_id == "140" {
                best_audio = Some(format.clone());
                break;
            }*/
            if format.format_id == "141" {
                best_audio = Some(format.clone());
            }
        }
        best_audio
    }
}

/// Get song info using yt-dlp
pub(crate) async fn get_video_info(
    video_id: &VideoId,
    access_token: &Option<String>,
) -> Result<YtdlVideoInfo> {
    let video_url = video_id.to_url();
    let mut args = vec![&video_url, "-j"];
    let output = if let Some(access_token) = access_token {
        args.push("--add-headers");
        let auth = format!("Authorization: Bearer {}", access_token);
        args.push(&auth);
        Command::new("yt-dlp").args(args).output().await?
    } else {
        Command::new("yt-dlp").args(args).output().await?
    };

    if output.status.success() {
        let json: YtdlVideoInfo = serde_json::from_str(&String::from_utf8(output.stdout)?)?;
        Ok(json)
    } else {
        let stderr = std::str::from_utf8(&output.stderr);
        Err(anyhow::anyhow!("failed to get video info: {:?}", stderr))
    }
}

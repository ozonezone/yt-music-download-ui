use anyhow::Context;
use once_cell::sync::Lazy;
use openapi::models::{ArtistRun, LikeStatus, PlaylistItem, QueueTrack, Thumbnail, VideoType};
use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct CommonTrack {
    pub video_id: String,
    pub title: String,
    pub artists: Vec<ArtistRun>,
    pub album: Option<String>,
    pub thumbnails: Vec<Thumbnail>,
    pub like_status: Option<LikeStatus>,
    pub video_type: Option<VideoType>,
    pub year: Option<String>,
}

impl TryFrom<PlaylistItem> for CommonTrack {
    type Error = anyhow::Error;
    fn try_from(track: PlaylistItem) -> Result<Self, Self::Error> {
        Ok(Self {
            video_id: track.video_id.context("video_id")?,
            title: track.title,
            artists: track.artists,
            album: track.album.map(|x| x.name),
            thumbnails: track.thumbnails.unwrap_or_default(),
            like_status: track.like_status,
            video_type: track.video_type,
            year: None,
        })
    }
}

impl From<QueueTrack> for CommonTrack {
    fn from(track: QueueTrack) -> Self {
        Self {
            video_id: track.video_id,
            title: track.title,
            artists: track.artists,
            album: track.album.map(|x| x.name),
            thumbnails: track.thumbnails,
            like_status: track.like_status,
            video_type: track.video_type,
            year: track.year,
        }
    }
}

static THUMBNAIL_SIZE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(.+=)w\d+-h\d+(.+)").unwrap());

impl CommonTrack {
    /// Get best quality thumbnail of track
    pub fn extract_best_thumbnail(&self) -> Option<Thumbnail> {
        let mut best_thumbnail: Option<&Thumbnail> = None;
        for thumbnail in &self.thumbnails {
            let (best_width, _best_height) = match &best_thumbnail {
                Some(best_thumbnail) => (best_thumbnail.width, best_thumbnail.height),
                None => (0.0, 0.0),
            };

            if thumbnail.width > best_width {
                best_thumbnail = Some(thumbnail);
            }
        }
        let best_thumbnail = best_thumbnail?;

        let t = match THUMBNAIL_SIZE_RE.replace(&best_thumbnail.url, "${1}w2000-h2000${2}") {
            std::borrow::Cow::Owned(url) => Thumbnail {
                url,
                width: best_thumbnail.width,
                height: best_thumbnail.height,
            },
            std::borrow::Cow::Borrowed(_) => best_thumbnail.clone(),
        };
        Some(t)
    }
}

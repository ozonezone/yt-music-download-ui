use openapi::models::{
    Album, ArtistRun, LikeStatus, PlaylistItem, QueueTrack, Thumbnail, VideoType,
};
use serde::Deserialize;

pub mod playlist;
pub mod queue;

#[derive(Deserialize, Clone)]
pub struct CommonTrack {
    pub video_id: String,
    pub title: String,
    pub artists: Vec<ArtistRun>,
    pub album: Option<Album>,
    pub thumbnails: Vec<Thumbnail>,
    pub like_status: Option<LikeStatus>,
    pub video_type: Option<VideoType>,
    pub year: Option<String>,
}

impl From<PlaylistItem> for CommonTrack {
    fn from(track: PlaylistItem) -> Self {
        Self {
            video_id: track.video_id,
            title: track.title,
            artists: track.artists,
            album: track.album.map(|x| *x),
            thumbnails: track.thumbnails,
            like_status: Some(track.like_status),
            video_type: Some(track.video_type),
            year: None,
        }
    }
}

impl From<QueueTrack> for CommonTrack {
    fn from(track: QueueTrack) -> Self {
        Self {
            video_id: track.video_id,
            title: track.title,
            artists: track.artists,
            album: track.album.map(|x| *x),
            thumbnails: track.thumbnails,
            like_status: track.like_status,
            video_type: track.video_type,
            year: track.year,
        }
    }
}

impl CommonTrack {
    /// Get best quality thumbnail of track
    pub fn extract_best_thumbnail(&self) -> Option<&Thumbnail> {
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
        best_thumbnail
    }
}

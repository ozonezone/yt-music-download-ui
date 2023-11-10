use serde::Deserialize;

use self::{playlist::PlaylistTrack, queue::QueueTrack};

pub mod playlist;
pub mod queue;

#[allow(clippy::enum_variant_names)]
#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VideoType {
    MusicVideoTypeOmv,
    MusicVideoTypeUgc,
    MusicVideoTypeAtv,
    MusicVideoTypePrivatelyOwnedTrack,
}
impl VideoType {
    pub fn is_music(&self) -> bool {
        matches!(
            self,
            VideoType::MusicVideoTypeAtv | VideoType::MusicVideoTypePrivatelyOwnedTrack
        )
    }
}

#[derive(Deserialize, Clone)]
pub struct Thumbnail {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Deserialize, Clone)]
pub struct Album {
    pub name: String,
    pub id: String,
}

#[derive(Deserialize, Clone)]
pub struct Artist {
    pub name: String,
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub artist_type: ArtistType,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ArtistType {
    Artist,
    Channel,
}

#[derive(Deserialize, Clone)]
pub enum LikeStatus {
    #[serde(rename = "INDIFFERENT")]
    Indifferent,
    #[serde(rename = "LIKE")]
    Like,
    #[serde(rename = "DISLIKE")]
    Dislike,
}

#[derive(Deserialize, Clone)]
pub struct CommonTrack {
    pub video_id: String,
    pub title: String,
    pub artists: Vec<Artist>,
    pub album: Option<Album>,
    pub thumbnails: Vec<Thumbnail>,
    pub like_status: LikeStatus,
    pub video_type: VideoType,
    pub duration: String,
    #[serde(rename = "duration_seconds")]
    pub duration_seconds: i64,
    pub year: Option<String>,
}

impl From<QueueTrack> for CommonTrack {
    fn from(track: QueueTrack) -> Self {
        Self {
            video_id: track.video_id,
            title: track.title,
            artists: track.artists,
            album: track.album,
            thumbnails: track.thumbnails,
            like_status: track.like_status,
            video_type: track.video_type,
            duration: track.duration,
            duration_seconds: track.duration_seconds,
            year: track.year,
        }
    }
}

impl From<PlaylistTrack> for CommonTrack {
    fn from(track: PlaylistTrack) -> Self {
        Self {
            video_id: track.video_id,
            title: track.title,
            artists: track.artists,
            album: track.album,
            thumbnails: track.thumbnails,
            like_status: track.like_status,
            video_type: track.video_type,
            duration: track.duration,
            duration_seconds: track.duration_seconds,
            year: None,
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
                None => (0, 0),
            };

            if thumbnail.width > best_width {
                best_thumbnail = Some(thumbnail);
            }
        }
        best_thumbnail
    }
}

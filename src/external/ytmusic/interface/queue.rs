use serde::Deserialize;

use super::{Album, Artist, LikeStatus, Thumbnail, VideoType};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Queue {
    pub chips: Vec<QueueChip>,
    pub playlist: Option<String>,
    pub playlist_id: Option<String>,
    pub tracks: Vec<QueueTrack>,
    pub lyrics: Option<String>,
    pub related: String,
    pub author: QueueAuthor,
    // pub continuation: String,
    pub current: QueueCurrent,
}

#[derive(Deserialize)]
pub struct QueueAuthor {
    pub name: Option<String>,
    pub id: Option<String>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueChip {
    pub title: String,
    pub playlist_id: String,
    pub params: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueCurrent {
    pub video_id: String,
    pub playlist_id: String,
    pub index: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueTrack {
    pub artists: Vec<Artist>,
    pub album: Option<Album>,
    pub views: Option<String>,
    pub duration: String,
    #[serde(rename = "duration_seconds")]
    pub duration_seconds: i64,
    pub year: Option<String>,
    pub video_id: String,
    pub title: String,
    pub thumbnails: Vec<Thumbnail>,
    // pub feedback_tokens: Option<QueueFeedbackTokens>,
    pub like_status: LikeStatus,
    pub video_type: VideoType,
    pub is_explicit: bool,
    pub counterpart: Option<Box<QueueTrack>>,
}

// #[derive(Deserialize)]
// pub struct QueueFeedbackTokens {
//     pub add: String,
//     pub remove: String,
//     pub saved: bool,
// }

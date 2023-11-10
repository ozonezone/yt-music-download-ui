use serde::Deserialize;

use super::{Album, Artist, LikeStatus, Thumbnail, VideoType};

#[derive(Deserialize)]
pub struct Playlist {
    pub id: String,
    pub privacy: PlaylistPrivacy,
    pub editable: bool,
    pub title: String,
    pub thumbnails: Vec<Thumbnail>,
    pub description: Option<serde_json::Value>,
    pub r#type: String,
    pub authors: Vec<Artist>,
    pub year: Option<String>,
    #[serde(rename = "trackCount")]
    pub track_count: String,
    pub duration: String,
    pub duration_seconds: i64,
    pub tracks: Vec<PlaylistTrack>,
    // pub continuation: Option<serde_json::Value>,
    // pub suggestions: Vec<Option<serde_json::Value>>,
    // pub suggestions_continuation: Option<serde_json::Value>,
    // pub related: Vec<Option<serde_json::Value>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlaylistPrivacy {
    Public,
    Private,
    Unlisted,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistTrack {
    pub video_id: String,
    pub title: String,
    pub artists: Vec<Artist>,
    pub album: Option<Album>,
    pub like_status: LikeStatus,
    pub thumbnails: Vec<Thumbnail>,
    pub is_available: bool,
    pub is_explicit: bool,
    pub video_type: VideoType,
    pub duration: String,
    #[serde(rename = "duration_seconds")]
    pub duration_seconds: i64,
    // pub set_video_id: String,
    // pub feedback_tokens: Option<FeedbackTokens>,
    // pub feedback_token: Option<serde_json::Value>,
    // pub rank: Option<serde_json::Value>,
    // pub change: Option<serde_json::Value>,
}

// #[derive(Deserialize)]
// pub struct FeedbackTokens {
//     pub add: String,
//     pub remove: String,
//     pub saved: bool,
// }

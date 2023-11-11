/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Playlist {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "privacy")]
    pub privacy: Privacy,
    #[serde(rename = "editable")]
    pub editable: bool,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "thumbnails")]
    pub thumbnails: Vec<crate::models::Thumbnail>,
    #[serde(rename = "description", deserialize_with = "Option::deserialize")]
    pub description: Option<String>,
    #[serde(rename = "authors")]
    pub authors: Vec<crate::models::ArtistRun>,
    /// can be `Playlist`, `Chart` or `Radio`
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "year", deserialize_with = "Option::deserialize")]
    pub year: Option<String>,
    #[serde(rename = "trackCount", deserialize_with = "Option::deserialize")]
    pub track_count: Option<serde_json::Value>,
    #[serde(rename = "duration", deserialize_with = "Option::deserialize")]
    pub duration: Option<String>,
    #[serde(rename = "duration_seconds")]
    pub duration_seconds: f64,
    #[serde(rename = "tracks")]
    pub tracks: Vec<crate::models::PlaylistItem>,
    #[serde(rename = "continuation", deserialize_with = "Option::deserialize")]
    pub continuation: Option<String>,
    #[serde(rename = "suggestions")]
    pub suggestions: Vec<crate::models::PlaylistItem>,
    #[serde(rename = "suggestions_continuation", deserialize_with = "Option::deserialize")]
    pub suggestions_continuation: Option<String>,
    #[serde(rename = "related")]
    pub related: Vec<crate::models::ParsedPlaylist>,
}

impl Playlist {
    pub fn new(id: String, privacy: Privacy, editable: bool, title: String, thumbnails: Vec<crate::models::Thumbnail>, description: Option<String>, authors: Vec<crate::models::ArtistRun>, r#type: String, year: Option<String>, track_count: Option<serde_json::Value>, duration: Option<String>, duration_seconds: f64, tracks: Vec<crate::models::PlaylistItem>, continuation: Option<String>, suggestions: Vec<crate::models::PlaylistItem>, suggestions_continuation: Option<String>, related: Vec<crate::models::ParsedPlaylist>) -> Playlist {
        Playlist {
            id,
            privacy,
            editable,
            title,
            thumbnails,
            description,
            authors,
            r#type,
            year,
            track_count,
            duration,
            duration_seconds,
            tracks,
            continuation,
            suggestions,
            suggestions_continuation,
            related,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Privacy {
    #[serde(rename = "PUBLIC")]
    Public,
    #[serde(rename = "PRIVATE")]
    Private,
    #[serde(rename = "UNLISTED")]
    Unlisted,
}

impl Default for Privacy {
    fn default() -> Privacy {
        Self::Public
    }
}


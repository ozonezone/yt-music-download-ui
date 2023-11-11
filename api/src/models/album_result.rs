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
pub struct AlbumResult {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "album_type")]
    pub album_type: String,
    #[serde(rename = "thumbnails")]
    pub thumbnails: Vec<crate::models::Thumbnail>,
    #[serde(rename = "isExplicit")]
    pub is_explicit: bool,
    #[serde(rename = "description", deserialize_with = "Option::deserialize")]
    pub description: Option<String>,
    #[serde(rename = "trackCount", deserialize_with = "Option::deserialize")]
    pub track_count: Option<String>,
    #[serde(rename = "duration", deserialize_with = "Option::deserialize")]
    pub duration: Option<String>,
    #[serde(rename = "audioPlaylistId", deserialize_with = "Option::deserialize")]
    pub audio_playlist_id: Option<String>,
    #[serde(rename = "likeStatus", deserialize_with = "Option::deserialize")]
    pub like_status: Option<crate::models::LikeStatus>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "tracks")]
    pub tracks: Vec<crate::models::PlaylistItem>,
    #[serde(rename = "other_versions", deserialize_with = "Option::deserialize")]
    pub other_versions: Option<Vec<crate::models::ParsedAlbum>>,
    #[serde(rename = "artists", skip_serializing_if = "Option::is_none")]
    pub artists: Option<Vec<crate::models::ArtistRun>>,
}

impl AlbumResult {
    pub fn new(title: String, album_type: String, thumbnails: Vec<crate::models::Thumbnail>, is_explicit: bool, description: Option<String>, track_count: Option<String>, duration: Option<String>, audio_playlist_id: Option<String>, like_status: Option<crate::models::LikeStatus>, id: String, tracks: Vec<crate::models::PlaylistItem>, other_versions: Option<Vec<crate::models::ParsedAlbum>>) -> AlbumResult {
        AlbumResult {
            title,
            album_type,
            thumbnails,
            is_explicit,
            description,
            track_count,
            duration,
            audio_playlist_id,
            like_status,
            id,
            tracks,
            other_versions,
            artists: None,
        }
    }
}



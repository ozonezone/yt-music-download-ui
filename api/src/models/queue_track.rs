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
pub struct QueueTrack {
    #[serde(rename = "artists")]
    pub artists: Vec<crate::models::ArtistRun>,
    #[serde(rename = "album", deserialize_with = "Option::deserialize")]
    pub album: Option<Box<crate::models::Album>>,
    #[serde(rename = "views", deserialize_with = "Option::deserialize")]
    pub views: Option<String>,
    #[serde(rename = "duration", deserialize_with = "Option::deserialize")]
    pub duration: Option<String>,
    #[serde(rename = "duration_seconds", deserialize_with = "Option::deserialize")]
    pub duration_seconds: Option<f64>,
    #[serde(rename = "year", deserialize_with = "Option::deserialize")]
    pub year: Option<String>,
    #[serde(rename = "videoId")]
    pub video_id: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "thumbnails")]
    pub thumbnails: Vec<crate::models::Thumbnail>,
    #[serde(rename = "feedbackTokens", deserialize_with = "Option::deserialize")]
    pub feedback_tokens: Option<Box<crate::models::MenuTokens>>,
    #[serde(rename = "likeStatus", deserialize_with = "Option::deserialize")]
    pub like_status: Option<crate::models::LikeStatus>,
    #[serde(rename = "videoType", deserialize_with = "Option::deserialize")]
    pub video_type: Option<crate::models::VideoType>,
    #[serde(rename = "isExplicit")]
    pub is_explicit: bool,
    #[serde(rename = "counterpart", deserialize_with = "Option::deserialize")]
    pub counterpart: Option<Box<crate::models::QueueTrack>>,
}

impl QueueTrack {
    pub fn new(artists: Vec<crate::models::ArtistRun>, album: Option<crate::models::Album>, views: Option<String>, duration: Option<String>, duration_seconds: Option<f64>, year: Option<String>, video_id: String, title: String, thumbnails: Vec<crate::models::Thumbnail>, feedback_tokens: Option<crate::models::MenuTokens>, like_status: Option<crate::models::LikeStatus>, video_type: Option<crate::models::VideoType>, is_explicit: bool, counterpart: Option<crate::models::QueueTrack>) -> QueueTrack {
        QueueTrack {
            artists,
            album: if let Some(x) = album {Some(Box::new(x))} else {None},
            views,
            duration,
            duration_seconds,
            year,
            video_id,
            title,
            thumbnails,
            feedback_tokens: if let Some(x) = feedback_tokens {Some(Box::new(x))} else {None},
            like_status,
            video_type,
            is_explicit,
            counterpart: if let Some(x) = counterpart {Some(Box::new(x))} else {None},
        }
    }
}



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
pub struct Queue {
    #[serde(rename = "chips")]
    pub chips: Vec<crate::models::QueueChip>,
    #[serde(rename = "playlistId", deserialize_with = "Option::deserialize")]
    pub playlist_id: Option<String>,
    #[serde(rename = "playlist", deserialize_with = "Option::deserialize")]
    pub playlist: Option<String>,
    #[serde(rename = "tracks")]
    pub tracks: Vec<crate::models::QueueTrack>,
    #[serde(rename = "lyrics", deserialize_with = "Option::deserialize")]
    pub lyrics: Option<String>,
    #[serde(rename = "related", deserialize_with = "Option::deserialize")]
    pub related: Option<String>,
    #[serde(rename = "author", deserialize_with = "Option::deserialize")]
    pub author: Option<Box<crate::models::QueueAuthor>>,
    #[serde(rename = "continuation", deserialize_with = "Option::deserialize")]
    pub continuation: Option<String>,
    #[serde(rename = "current", deserialize_with = "Option::deserialize")]
    pub current: Option<Box<crate::models::QueueCurrent>>,
}

impl Queue {
    pub fn new(chips: Vec<crate::models::QueueChip>, playlist_id: Option<String>, playlist: Option<String>, tracks: Vec<crate::models::QueueTrack>, lyrics: Option<String>, related: Option<String>, author: Option<crate::models::QueueAuthor>, continuation: Option<String>, current: Option<crate::models::QueueCurrent>) -> Queue {
        Queue {
            chips,
            playlist_id,
            playlist,
            tracks,
            lyrics,
            related,
            author: if let Some(x) = author {Some(Box::new(x))} else {None},
            continuation,
            current: if let Some(x) = current {Some(Box::new(x))} else {None},
        }
    }
}



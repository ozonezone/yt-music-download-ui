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
pub struct QueueCurrent {
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<f64>,
    #[serde(rename = "playlistId", skip_serializing_if = "Option::is_none")]
    pub playlist_id: Option<String>,
    #[serde(rename = "videoId")]
    pub video_id: String,
}

impl QueueCurrent {
    pub fn new(video_id: String) -> QueueCurrent {
        QueueCurrent {
            index: None,
            playlist_id: None,
            video_id,
        }
    }
}



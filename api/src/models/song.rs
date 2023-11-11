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
pub struct Song {
    #[serde(rename = "formats")]
    pub formats: Vec<crate::models::Format>,
    #[serde(rename = "adaptive_formats")]
    pub adaptive_formats: Vec<crate::models::Format>,
    #[serde(rename = "expires")]
    pub expires: String,
    #[serde(rename = "videoDetails")]
    pub video_details: Box<crate::models::VideoDetails>,
    #[serde(rename = "playerConfig", deserialize_with = "Option::deserialize")]
    pub player_config: Option<serde_json::Value>,
    #[serde(rename = "playbackTracking", deserialize_with = "Option::deserialize")]
    pub playback_tracking: Option<serde_json::Value>,
    #[serde(rename = "videostatsPlaybackUrl")]
    pub videostats_playback_url: String,
    #[serde(rename = "captions")]
    pub captions: Vec<crate::models::Caption>,
    #[serde(rename = "hlsManifestUrl", deserialize_with = "Option::deserialize")]
    pub hls_manifest_url: Option<String>,
    #[serde(rename = "aspectRatio")]
    pub aspect_ratio: f64,
    #[serde(rename = "serverAbrStreamingUrl")]
    pub server_abr_streaming_url: String,
}

impl Song {
    pub fn new(formats: Vec<crate::models::Format>, adaptive_formats: Vec<crate::models::Format>, expires: String, video_details: crate::models::VideoDetails, player_config: Option<serde_json::Value>, playback_tracking: Option<serde_json::Value>, videostats_playback_url: String, captions: Vec<crate::models::Caption>, hls_manifest_url: Option<String>, aspect_ratio: f64, server_abr_streaming_url: String) -> Song {
        Song {
            formats,
            adaptive_formats,
            expires,
            video_details: Box::new(video_details),
            player_config,
            playback_tracking,
            videostats_playback_url,
            captions,
            hls_manifest_url,
            aspect_ratio,
            server_abr_streaming_url,
        }
    }
}


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
pub struct AudioFormat {
    #[serde(rename = "container")]
    pub container: Container,
    #[serde(rename = "projection_type")]
    pub projection_type: ProjectionType,
    #[serde(rename = "mime_type")]
    pub mime_type: String,
    #[serde(rename = "modified")]
    pub modified: String,
    #[serde(rename = "itag")]
    pub itag: f64,
    #[serde(rename = "init_range", deserialize_with = "Option::deserialize")]
    pub init_range: Option<Box<crate::models::VideoFormatInitRange>>,
    #[serde(rename = "index_range", deserialize_with = "Option::deserialize")]
    pub index_range: Option<Box<crate::models::VideoFormatInitRange>>,
    #[serde(rename = "content_length", deserialize_with = "Option::deserialize")]
    pub content_length: Option<f64>,
    #[serde(rename = "bitrate")]
    pub bitrate: f64,
    #[serde(rename = "average_bitrate", deserialize_with = "Option::deserialize")]
    pub average_bitrate: Option<f64>,
    #[serde(rename = "duration_ms")]
    pub duration_ms: f64,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "codecs")]
    pub codecs: String,
    #[serde(rename = "has_audio")]
    pub has_audio: bool,
    #[serde(rename = "has_video")]
    pub has_video: bool,
    #[serde(rename = "audio_quality")]
    pub audio_quality: AudioQuality,
    #[serde(rename = "channels")]
    pub channels: f64,
    #[serde(rename = "sample_rate")]
    pub sample_rate: f64,
    #[serde(rename = "audio_codec", deserialize_with = "Option::deserialize")]
    pub audio_codec: Option<String>,
    #[serde(rename = "quality")]
    pub quality: Quality,
}

impl AudioFormat {
    pub fn new(container: Container, projection_type: ProjectionType, mime_type: String, modified: String, itag: f64, init_range: Option<crate::models::VideoFormatInitRange>, index_range: Option<crate::models::VideoFormatInitRange>, content_length: Option<f64>, bitrate: f64, average_bitrate: Option<f64>, duration_ms: f64, url: String, codecs: String, has_audio: bool, has_video: bool, audio_quality: AudioQuality, channels: f64, sample_rate: f64, audio_codec: Option<String>, quality: Quality) -> AudioFormat {
        AudioFormat {
            container,
            projection_type,
            mime_type,
            modified,
            itag,
            init_range: if let Some(x) = init_range {Some(Box::new(x))} else {None},
            index_range: if let Some(x) = index_range {Some(Box::new(x))} else {None},
            content_length,
            bitrate,
            average_bitrate,
            duration_ms,
            url,
            codecs,
            has_audio,
            has_video,
            audio_quality,
            channels,
            sample_rate,
            audio_codec,
            quality,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Container {
    #[serde(rename = "flv")]
    Flv,
    #[serde(rename = "3gp")]
    Variant3gp,
    #[serde(rename = "mp4")]
    Mp4,
    #[serde(rename = "webm")]
    Webm,
    #[serde(rename = "ts")]
    Ts,
}

impl Default for Container {
    fn default() -> Container {
        Self::Flv
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProjectionType {
    #[serde(rename = "360")]
    Variant360,
    #[serde(rename = "rectangular")]
    Rectangular,
    #[serde(rename = "stereoscopic")]
    Stereoscopic,
    #[serde(rename = "3d")]
    Variant3d,
}

impl Default for ProjectionType {
    fn default() -> ProjectionType {
        Self::Variant360
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AudioQuality {
    #[serde(rename = "tiny")]
    Tiny,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
}

impl Default for AudioQuality {
    fn default() -> AudioQuality {
        Self::Tiny
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Quality {
    #[serde(rename = "tiny")]
    Tiny,
    #[serde(rename = "small")]
    Small,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "large")]
    Large,
}

impl Default for Quality {
    fn default() -> Quality {
        Self::Tiny
    }
}


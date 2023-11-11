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
pub struct VideoDetailsThumbnail {
    #[serde(rename = "thumbnails")]
    pub thumbnails: Vec<crate::models::Thumbnail>,
}

impl VideoDetailsThumbnail {
    pub fn new(thumbnails: Vec<crate::models::Thumbnail>) -> VideoDetailsThumbnail {
        VideoDetailsThumbnail {
            thumbnails,
        }
    }
}



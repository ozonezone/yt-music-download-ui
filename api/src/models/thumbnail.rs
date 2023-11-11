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
pub struct Thumbnail {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "width")]
    pub width: f64,
    #[serde(rename = "height")]
    pub height: f64,
}

impl Thumbnail {
    pub fn new(url: String, width: f64, height: f64) -> Thumbnail {
        Thumbnail {
            url,
            width,
            height,
        }
    }
}



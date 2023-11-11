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
pub struct VideoFormatInitRange {
    #[serde(rename = "start")]
    pub start: f64,
    #[serde(rename = "end")]
    pub end: f64,
}

impl VideoFormatInitRange {
    pub fn new(start: f64, end: f64) -> VideoFormatInitRange {
        VideoFormatInitRange {
            start,
            end,
        }
    }
}



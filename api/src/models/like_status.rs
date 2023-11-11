/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LikeStatus {
    #[serde(rename = "LIKE")]
    Like,
    #[serde(rename = "INDIFFERENT")]
    Indifferent,
    #[serde(rename = "DISLIKE")]
    Dislike,

}

impl ToString for LikeStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Like => String::from("LIKE"),
            Self::Indifferent => String::from("INDIFFERENT"),
            Self::Dislike => String::from("DISLIKE"),
        }
    }
}

impl Default for LikeStatus {
    fn default() -> LikeStatus {
        Self::Like
    }
}





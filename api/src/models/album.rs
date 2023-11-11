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
pub struct Album {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "id", deserialize_with = "Option::deserialize")]
    pub id: Option<String>,
}

impl Album {
    pub fn new(name: String, id: Option<String>) -> Album {
        Album {
            name,
            id,
        }
    }
}



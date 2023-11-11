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
pub struct QueueAuthor {
    #[serde(rename = "id", deserialize_with = "Option::deserialize")]
    pub id: Option<String>,
    #[serde(rename = "name", deserialize_with = "Option::deserialize")]
    pub name: Option<String>,
}

impl QueueAuthor {
    pub fn new(id: Option<String>, name: Option<String>) -> QueueAuthor {
        QueueAuthor {
            id,
            name,
        }
    }
}



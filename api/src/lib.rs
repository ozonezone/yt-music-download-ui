#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

pub mod apis;
pub mod models;

impl models::VideoType {
    pub fn is_music(&self) -> bool {
        matches!(
            self,
            models::VideoType::Atv | models::VideoType::PrivatelyOwnedTrack
        )
    }
}

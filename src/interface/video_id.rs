use std::fmt::{Display, Formatter};

use anyhow::{Context, Result};

pub(crate) struct VideoId {
    pub id: String,
}

impl Display for VideoId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl VideoId {
    pub fn from_url(url: &str) -> Result<Self> {
        let id = url
            .split("v=")
            .last()
            .with_context(|| "No video id found: 01")?
            .split('&')
            .next()
            .with_context(|| "No video id found: 02")?;
        Ok(VideoId { id: id.to_string() })
    }
    pub fn from_id(id: &str) -> Self {
        VideoId { id: id.to_string() }
    }

    pub fn from_amibgous_url(url: &str) -> Self {
        if let Ok(id) = VideoId::from_url(url) {
            id
        } else {
            VideoId::from_id(url)
        }
    }

    pub fn to_url(&self) -> String {
        format!("https://music.youtube.com/watch?v={}", self.id)
    }
}

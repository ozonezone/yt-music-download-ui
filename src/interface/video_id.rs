use std::fmt::{Display, Formatter};

use anyhow::Result;

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
        let Ok(url) = url::Url::parse(url) else {
            return Err(anyhow::anyhow!("Failed to parse url"));
        };
        let Some(video_id) = url.query_pairs().find(|q| q.0 == "v").map(|q| Self {
            id: q.1.to_string(),
        }) else {
            return Err(anyhow::anyhow!("Failed to find playlist id from url"));
        };

        Ok(video_id)
    }
    pub fn from_id(id: &str) -> Self {
        VideoId { id: id.to_string() }
    }
    pub fn from_id_or_url(url: &str) -> Self {
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

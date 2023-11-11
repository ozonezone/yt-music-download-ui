use anyhow::Result;
use std::fmt::{Display, Formatter};

pub(crate) struct PlaylistId {
    pub id: String,
}

impl Display for PlaylistId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl PlaylistId {
    pub fn from_url(url: &str) -> Result<Self> {
        let Ok(url) = url::Url::parse(url) else {
            return Err(anyhow::anyhow!("Failed to parse url"));
        };
        let Some(playlist_id) = url.query_pairs().find(|q| q.0 == "list").map(|q| Self {
            id: q.1.to_string(),
        }) else {
            return Err(anyhow::anyhow!("Failed to find playlist id from url"));
        };

        Ok(playlist_id)
    }
    pub fn from_id(id: &str) -> Self {
        PlaylistId { id: id.to_string() }
    }
    pub fn from_id_or_url(str: &str) -> Self {
        if let Ok(id) = PlaylistId::from_url(str) {
            id
        } else {
            PlaylistId::from_id(str)
        }
    }

    pub fn to_url(&self) -> String {
        format!("https://music.youtube.com/playlist?list={}", self.id)
    }
}

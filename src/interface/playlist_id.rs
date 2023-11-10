use anyhow::{Context, Result};
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
        let id = url
            .split("list=")
            .last()
            .with_context(|| "No video id found: 01")?
            .split('&')
            .next()
            .with_context(|| "No video id found: 02")?;
        Ok(PlaylistId { id: id.to_string() })
    }
    pub fn from_id(id: &str) -> Self {
        PlaylistId { id: id.to_string() }
    }
    pub fn to_url(&self) -> String {
        format!("https://music.youtube.com/playlist?list={}", self.id)
    }
    pub fn from_ambigous_url(url: &str) -> Self {
        if let Ok(id) = PlaylistId::from_url(url) {
            id
        } else {
            PlaylistId::from_id(url)
        }
    }
}

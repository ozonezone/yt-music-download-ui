use anyhow::Result;
use std::fmt::{Display, Formatter};

pub(crate) struct BrowseId {
    pub id: String,
}

impl Display for BrowseId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl BrowseId {
    pub fn from_url(url: &str) -> Result<Self> {
        let Ok(url) = url::Url::parse(url) else {
            return Err(anyhow::anyhow!("Failed to parse url"));
        };
        let Some(segments) = url.path_segments() else {
            return Err(anyhow::anyhow!("Failed to find browse id from url"));
        };

        let Some(playlist_id) = segments.last() else {
            return Err(anyhow::anyhow!("Failed to find browse id from url"));
        };

        Ok(Self {
            id: playlist_id.to_string(),
        })
    }
    pub fn from_id(id: &str) -> Self {
        BrowseId { id: id.to_string() }
    }
    pub fn from_id_or_url(str: &str) -> Self {
        if let Ok(id) = BrowseId::from_url(str) {
            id
        } else {
            BrowseId::from_id(str)
        }
    }
}

use openapi::{
    apis::{default_api::GetSongError, Error},
    models::{Song, SongGetParams},
};

use crate::{
    config::{API_CONFIG, CONFIG},
    interface::video_id::VideoId,
};

/// Get radio of song using ytmusicapi
pub(crate) async fn get_song(video_id: &VideoId) -> Result<Song, Error<GetSongError>> {
    openapi::apis::default_api::get_song(
        &API_CONFIG,
        SongGetParams {
            video_id: video_id.id.clone(),
            auth_path: format!("{}/auth.json", CONFIG.config_path),
            language: CONFIG.language.map(|s| s.to_string()),
        },
    )
    .await
}

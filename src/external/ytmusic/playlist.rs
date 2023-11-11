use openapi::{
    apis::{default_api::GetPlaylistError, Error},
    models::{Playlist, PlaylistGetParams},
};

use crate::{
    config::{API_CONFIG, CONFIG},
    interface::playlist_id::PlaylistId,
};

/// Get playlist
pub(crate) async fn get_playlist(
    playlist_id: &PlaylistId,
) -> Result<Playlist, Error<GetPlaylistError>> {
    openapi::apis::default_api::get_playlist(
        &API_CONFIG,
        PlaylistGetParams {
            playlist_id: playlist_id.id.clone(),
            auth_path: format!("{}/auth.json", CONFIG.config_path),
            language: CONFIG.language.map(|s| s.to_string()),
        },
    )
    .await
}

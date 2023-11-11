use openapi::{
    apis::{default_api::GetPlaylistError, Error},
    models::{Playlist, PlaylistGetParams},
};

use crate::{
    constants::{API_CONFIG, AUTH_FILE},
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
            auth_path: AUTH_FILE.to_string(),
        },
    )
    .await
}

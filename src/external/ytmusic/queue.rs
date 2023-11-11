use openapi::{
    apis::{default_api::GetQueueError, Error},
    models::{Queue, QueueGetParams},
};

use crate::{
    config::{API_CONFIG, CONFIG},
    interface::video_id::VideoId,
};

/// Get radio of song using ytmusicapi
pub(crate) async fn get_queue(
    video_id: &VideoId,
    radio: bool,
) -> Result<Queue, Error<GetQueueError>> {
    openapi::apis::default_api::get_queue(
        &API_CONFIG,
        QueueGetParams {
            video_id: video_id.id.clone(),
            radio,
            auth_path: format!("{}/auth.json", CONFIG.config_path),
            language: CONFIG.language.map(|s| s.to_string()),
        },
    )
    .await
}

use crate::{
    config::CONFIG,
    external::{
        ytdl::get_video_info,
        ytmusic::{song::get_song, CommonTrack},
    },
    interface::video_id::VideoId,
};
use anyhow::{Context, Result};
use futures::StreamExt;
use mp4ameta::{Data, FreeformIdent};
use std::io::Write;

fn sanitize(s: &str) -> String {
    sanitize_filename::sanitize_with_options(
        s,
        sanitize_filename::Options {
            windows: false,
            truncate: false,
            replacement: " ",
        },
    )
}

pub(crate) async fn download_track(
    track: &CommonTrack,
    overwrite: bool,
    track_number: Option<(u16, u16)>,
    write_youtube_id: bool,
    access_token: &Option<String>,
) -> Result<()> {
    let thumbnail = track
        .extract_best_thumbnail()
        .context("No thumbnail found")?;

    let video_id = VideoId::from_id(&track.video_id);

    let album = track.album.clone().context("No album found")?;
    let artist = track
        .artists
        .first()
        .context("No artist found")?
        .name
        .clone();

    let path_str = format!(
        "./{}/{}/{}/{}.m4a",
        CONFIG.download_path,
        sanitize(&artist),
        sanitize(&album.name),
        sanitize(&track.title)
    );
    let path = std::path::Path::new(&path_str);

    if !path.exists() || overwrite {
        let info = get_video_info(&video_id, access_token).await?;
        let best_audio = info.get_best_audio().context("no audio found.")?;

        tokio::fs::create_dir_all(path.parent().expect("what")).await?;

        let mut file = std::fs::File::create(path)?;

        let client = reqwest::Client::new();
        let mut req = client.get(best_audio.url);
        if let Some(token) = access_token {
            req = req.header(reqwest::header::AUTHORIZATION, format!("Bearer {}", token));
        }
        let mut stream = req.send().await?.bytes_stream();
        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result?;
            file.write_all(&chunk)?;
        }
        file.flush()?;

        let thumb_res = client.get(&thumbnail.url).send().await?;
        let thumb_type = thumb_res
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .context("No content type")?
            .to_str()?
            .to_string();
        let thumb_bin = thumb_res.bytes().await?;

        let tag_img = if thumb_type == "image/jpeg" {
            mp4ameta::Img::jpeg(thumb_bin.to_vec())
        } else {
            mp4ameta::Img::png(thumb_bin.to_vec())
        };
        let mut tag = mp4ameta::Tag::read_from_path(path)?;
        tag.set_artist(&artist);
        tag.set_album(&album.name);
        tag.set_title(&track.title);
        tag.set_artwork(tag_img);
        if let Some(year) = &track.year {
            tag.set_year(year);
        }
        if let Some(track_number) = track_number {
            tag.set_track(track_number.0, track_number.1);
        };
        if write_youtube_id {
            tag.set_data(
                FreeformIdent::new("youtube_id", "youtube_id"),
                Data::Utf8(track.video_id.clone()),
            );
        }
        tag.write_to_path(path).unwrap();
    } else {
        return Err(anyhow::anyhow!("Skipped. File already exists."));
    }

    Ok(())
}

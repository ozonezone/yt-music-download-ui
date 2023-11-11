use crate::{
    constants::DOWNLOAD_PATH,
    external::{ytdl::get_video_info, ytmusic::CommonTrack},
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
    let mut info = None;

    let (album, artist) = {
        let first_artist = track.artists.first();
        if let (Some(album), Some(artist)) = (&track.album, first_artist) {
            (album.name.clone(), artist.name.clone())
        } else {
            let i = get_video_info(&video_id, access_token).await?;
            let r = match (&i.album, &i.artist) {
                (Some(album), Some(artist)) => (album.clone(), artist.clone()),
                _ => {
                    return Err(anyhow::anyhow!(
                        "No album or artist found for {}",
                        track.video_id
                    ))
                }
            };
            info = Some(i);
            r
        }
    };

    let path_str = format!(
        "./{DOWNLOAD_PATH}/{}/{}/{}.m4a",
        sanitize(&artist),
        sanitize(&album),
        sanitize(&track.title)
    );
    let path = std::path::Path::new(&path_str);

    if !path.exists() || overwrite {
        let info = if let Some(info) = info {
            info
        } else {
            get_video_info(&video_id, access_token).await?
        };
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
        tag.set_album(&album);
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

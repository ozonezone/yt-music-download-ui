use crate::{
    external::{ytdl::download_video, ytmusic::CommonTrack},
    interface::video_id::VideoId,
};
use anyhow::{Context, Result};
use mp4ameta::{Data, FreeformIdent};
use std::path::Path;

pub struct DownloadOpts {
    pub track_number: Option<(u16, u16)>,
    pub overwrite: bool,
    pub write_youtube_id: bool,
}
pub(crate) async fn download_track(
    track: &CommonTrack,
    access_token: &Option<String>,
    path: &Path,
    opts: DownloadOpts,
) -> Result<()> {
    let thumbnail = track
        .extract_best_thumbnail()
        .context("No thumbnail found")?;

    let video_id = VideoId::from_id(&track.video_id);

    let path = std::path::Path::new(&path);

    if !path.exists() || opts.overwrite {
        let temp_file = tempfile::Builder::new().suffix(".m4a").tempfile()?;
        let ytdl_result = download_video(&video_id, access_token, &temp_file).await?;
        println!("{}", ytdl_result);

        let client = reqwest::Client::new();
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
        let mut tag = mp4ameta::Tag::read_from_path(&temp_file)?;

        let album = track.album.as_ref().context("No album found")?;
        let artist = track
            .artists
            .iter()
            .map(|x| x.name.clone())
            .collect::<Vec<_>>()
            .join(", ");
        tag.set_artist(&artist);
        tag.set_album(album.clone());
        tag.set_title(&track.title);
        tag.set_artwork(tag_img);
        if let Some(year) = &track.year {
            tag.set_year(year);
        }
        if let Some(track_number) = opts.track_number {
            tag.set_track(track_number.0, track_number.1);
        };
        if opts.write_youtube_id {
            tag.set_data(
                FreeformIdent::new("youtube_id", "youtube_id"),
                Data::Utf8(track.video_id.clone()),
            );
        }
        tag.write_to_path(&temp_file)?;

        tokio::fs::create_dir_all(path.parent().expect("what")).await?;
        tokio::fs::copy(&temp_file, path).await?;
    } else {
        return Err(anyhow::anyhow!("Skipped. File already exists."));
    }

    Ok(())
}

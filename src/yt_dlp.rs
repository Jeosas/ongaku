use core::str;
use std::process::Command;

use log::{debug, info};
use serde::Deserialize;

use crate::{
    db::{entry::EntryType, generate_id, Entry},
    error::OngakuError,
};

#[derive(Debug, Deserialize)]
struct UrlInfoExtractor {
    channel_id: String,
    channel: String,
    channel_url: String,
}

fn is_ytmusic_channel_url(url: &str) -> bool {
    info!("Checking url is YTMusic Channel");
    url.starts_with("https://music.youtube.com/channel/")
}

pub fn get_url_info(url: &str) -> Result<Entry, OngakuError> {
    let mut base_command = Command::new("yt-dlp");
    base_command
        .arg("--quiet")
        .arg("--flat-playlist")
        .arg("--print")
        .arg("%()j");

    match url {
        url if is_ytmusic_channel_url(url) => {
            info!("Fetching YTMusic channel data");
            let output = base_command
                .arg(url)
                .output()
                .map_err(|e| OngakuError::YtdlpError(e.to_string()))?;
            if !output.status.success() {
                return Err(OngakuError::YtdlpError(
                    str::from_utf8(&output.stderr).unwrap().to_string(),
                ));
            }

            let first_entry_raw = str::from_utf8(&output.stdout)
                .map_err(|e| OngakuError::YtdlpError(e.to_string()))?
                .trim()
                .split("\n")
                .next()
                .expect("always one output");

            info!("Parsing YTMusic channel data");
            debug!("{:?}", first_entry_raw);
            let first_entry: UrlInfoExtractor = serde_json::from_str(first_entry_raw)?;
            info!("Creating new Artist entry");
            Ok(Entry {
                id: generate_id(&first_entry.channel_id, &EntryType::Artist),
                original_url: url.to_string(),
                url: first_entry.channel_url,
                r#type: EntryType::Artist.into(),
                name: first_entry.channel.replace(" - Topic", ""),
                tracks: Vec::new(),
            })
        }
        _ => Err(OngakuError::UnsupportedUrl(url.to_owned())),
    }
}

#[derive(Debug, Deserialize)]
struct TrackExtractor {
    webpage_url: String,
}

pub fn get_tracks(url: &str) -> Result<Vec<String>, OngakuError> {
    let mut base_command = Command::new("yt-dlp");
    base_command
        .arg("--quiet")
        .arg("--flat-playlist")
        .arg("--print")
        .arg("%()j");

    info!("Fetching tracks");
    let output = base_command
        .arg(url)
        .output()
        .map_err(|e| OngakuError::YtdlpError(e.to_string()))?;
    if !output.status.success() {
        return Err(OngakuError::YtdlpError(
            str::from_utf8(&output.stderr).unwrap().to_string(),
        ));
    }

    let raw_tracks = str::from_utf8(&output.stdout)
        .map_err(|e| OngakuError::YtdlpError(e.to_string()))?
        .trim()
        .split("\n");

    info!("Parsing tracks");
    let mut tracks: Vec<String> = Vec::new();
    for track_raw in raw_tracks {
        debug!("{:?}", track_raw);
        let track: TrackExtractor = serde_json::from_str(track_raw)?;
        tracks.push(track.webpage_url);
    }
    Ok(tracks)
}

pub fn download_track(
    url: &str,
    entry_type: &EntryType,
    entry_name: &str,
) -> Result<String, OngakuError> {
    info!("Downloading '{}'", &url);
    let prefix = match entry_type {
        EntryType::Artist => format!("_artist/{}/", entry_name),
        EntryType::Playlist => format!("_playlists/{}/", entry_name),
        _ => "".to_string(),
    };

    let output = Command::new("yt-dlp")
        .arg("-q")
        .arg("-f")
        .arg("bestaudio/best")
        .arg("--extract-audio")
        .arg("-o")
        .arg(format!("{}%(title)s.%(ext)s", prefix))
        .arg("--print")
        .arg("after_move:filepath")
        .arg(&url)
        .output()
        .map_err(|e| OngakuError::YtdlpError(e.to_string()))?;
    if !output.status.success() {
        return Err(OngakuError::YtdlpError(
            str::from_utf8(&output.stderr).unwrap().to_string(),
        ));
    }

    let filepath = str::from_utf8(&output.stdout)
        .map_err(|e| OngakuError::YtdlpError(e.to_string()))?
        .trim()
        .split("\n")
        .next()
        .expect("always one output")
        .to_owned();

    Ok(filepath)
}

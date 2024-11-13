use core::str;
use std::process::Command;

use log::{debug, info};
use serde::Deserialize;

use crate::error::OngakuError;

const SUPPORTED_URL: &'static [&'static str] = &[
    "https://music.youtube.com/channel/",
    "https://music.youtube.com/watch?v=",
];

pub fn is_supported_url(url: &str) -> bool {
    SUPPORTED_URL.iter().any(|pattern| url.starts_with(pattern))
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

pub fn download_track(url: &str) -> Result<String, OngakuError> {
    info!("Downloading '{}'", &url);
    let output = Command::new("yt-dlp")
        .arg("-q")
        .arg("-f")
        .arg("bestaudio/best")
        .arg("--extract-audio")
        .arg("--add-metadata")
        .arg("-o")
        .arg("%(artists.0)#S/%(album|Unknown)#S/%(title)#S.%(ext)s")
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

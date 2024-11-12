use core::str;
use std::process::Command;

use log::debug;
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
    debug!("Checking url is YTMusic Channel");
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
            debug!("Fetching YTMusic channel data");
            let output = base_command
                .arg(url)
                .output()
                .map_err(|e| OngakuError::YtdlpError(e.to_string()))?;
            if !output.status.success() {
                return Err(OngakuError::YtdlpError(
                    str::from_utf8(&output.stderr).unwrap().to_string(),
                ));
            }

            match str::from_utf8(&output.stdout)
                .map_err(|e| OngakuError::YtdlpError(e.to_string()))?
                .split("\n")
                .next()
            {
                Some(first_entry_raw) => {
                    debug!("Parsing YTMusic channel data");
                    let first_entry: UrlInfoExtractor = serde_json::from_str(first_entry_raw)?;
                    debug!("Creating new Artist entry");
                    Ok(Entry {
                        id: generate_id(&first_entry.channel_id, &EntryType::Artist),
                        original_url: url.to_string(),
                        url: first_entry.channel_url,
                        r#type: EntryType::Artist.into(),
                        name: first_entry.channel.replace(" - Topic", ""),
                        tracks: Vec::new(),
                    })
                }
                None => Err(OngakuError::YtdlpError("no output.".to_string())),
            }
        }
        _ => Err(OngakuError::UnsupportedUrl(url.to_owned())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entry_from_ytmusic_channel() {
        let result = get_url_info("https://music.youtube.com/channel/UCzIVTMt4MpC3JNfcQtSAfCA");
        dbg!(&result);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Entry {
                id: "ARTIST/UCzIVTMt4MpC3JNfcQtSAfCA".to_string(),
                original_url: "https://music.youtube.com/channel/UCzIVTMt4MpC3JNfcQtSAfCA"
                    .to_string(),
                url: "https://www.youtube.com/channel/UCzIVTMt4MpC3JNfcQtSAfCA".to_string(),
                r#type: EntryType::Artist.into(),
                name: "Sam Long".to_string(),
                tracks: Vec::new(),
            }
        )
    }

    #[test]
    fn entry_from_unsupported_url() {
        get_url_info("https://www.rust-lang.org").unwrap_err();
    }
}

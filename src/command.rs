use console::{style, Emoji};
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use log::{error, info};
use std::{num::NonZero, sync::mpsc::channel, thread::available_parallelism};
use threadpool::ThreadPool;

use crate::{
    db::{self, Entry, Track},
    error::OngakuError,
    yt_dlp,
};

static SUCCESS: Emoji<'_, '_> = Emoji("✅", "");
static WARNING: Emoji<'_, '_> = Emoji("⚠️ ", "");
static INFO: Emoji<'_, '_> = Emoji(" ℹ️", "");

fn get_bar_style() -> ProgressStyle {
    ProgressStyle::with_template(
        "[{elapsed_precise}] {message} {bar:30.bold.cyan/.bold.dim} {pos:>7}/{len:7} ({eta})",
    )
    .expect("failed to parse bar style")
    .progress_chars("━━━")
}

fn get_spinner_style() -> ProgressStyle {
    ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
        .expect("failed to parse bar style")
        .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠇⠏")
}

pub fn init() -> Result<(), OngakuError> {
    info!("Running init command");
    db::init()?;
    println!("{} Successfully initialized library.", SUCCESS,);
    Ok(())
}

pub fn add(name: &str, url: &str) -> Result<(), OngakuError> {
    info!("Running add command");
    let mut library = db::load()?;

    info!("Checking url support");
    if !yt_dlp::is_supported_url(url) {
        return Err(OngakuError::UnsupportedUrl(url.to_owned()));
    }

    info!("Checking entry duplication");
    if library.entries.contains_key(name) {
        return Err(OngakuError::AlreadyInLibrary(name.to_owned()));
    }

    info!("Adding entry to library");
    library.entries.insert(
        name.to_owned(),
        Entry {
            url: url.to_owned(),
            name: name.to_owned(),
            tracks: Vec::new(),
        },
    );

    db::save(library)?;

    println!(
        "{} Successfully added {} to your library.",
        SUCCESS,
        style(name).cyan(),
    );
    println!(
        "{} Run {} to download tracks.",
        INFO,
        style("ongaku sync").cyan()
    );
    Ok(())
}

pub fn sync() -> Result<(), OngakuError> {
    info!("Running sync command");

    let bar_style = get_bar_style();
    let spinner_style = get_spinner_style();

    println!(
        "{} {} Loading database",
        style("[1/3]").bold().dim(),
        Emoji("💽", "")
    );
    let mut library = {
        let pb = ProgressBar::new(10)
            .with_style(spinner_style.clone())
            .with_message("Loading database");

        let library = db::load()?;

        pb.finish_and_clear();

        library
    };

    println!(
        "{} {} Listing missing tracks",
        style("[2/3]").bold().dim(),
        Emoji("🔍", "")
    );
    let mut tracks_in_lib = {
        let pb = ProgressBar::new(10)
            .with_style(spinner_style.clone())
            .with_message("Loading library");

        let tracks_in_lib: Vec<String> = library
            .entries
            .iter()
            .flat_map(|(_, e)| e.tracks.iter().map(|t| t.url.to_owned()))
            .collect();

        pb.finish_and_clear();

        tracks_in_lib
    };

    #[derive(Debug, Clone)]
    struct Task {
        entry_name: String,
        track_url: String,
    }

    let track_tasks = {
        let mut track_tasks: Vec<Task> = Vec::new();
        for (_, entry) in library
            .entries
            .iter()
            .progress_with_style(bar_style.clone())
            .with_message("Listing new tracks")
        {
            match yt_dlp::get_tracks(&entry.url) {
                Ok(track_urls) => {
                    for track_url in track_urls {
                        if tracks_in_lib.contains(&track_url) {
                            continue;
                        }
                        track_tasks.push(Task {
                            entry_name: entry.name.to_owned(),
                            track_url: track_url.to_owned(),
                        });
                        tracks_in_lib.push(track_url);
                    }
                }
                Err(e) => {
                    error!("Failed to fetch url ({}): {}", &entry.url, e.to_string());
                    eprintln!(
                        "{} {}",
                        WARNING,
                        style(format!(
                            "Failed to fetch tracks for {} ({})",
                            &entry.name, &entry.url
                        ))
                        .yellow()
                    );
                }
            }
        }

        track_tasks
    };

    println!(
        "{} {} Downloading missing tracks",
        style("[3/3]").bold().dim(),
        Emoji("📥", "")
    );
    {
        let pb = ProgressBar::new(track_tasks.len() as u64)
            .with_style(bar_style.clone())
            .with_message("Downloading tracks");
        pb.inc(0);

        info!("Creating thread pool");
        let cpu_count = available_parallelism()
            .unwrap_or(NonZero::new(1).expect("hardcoded value"))
            .get();
        let pool = ThreadPool::new(cpu_count);

        let (tx, rx) = channel();
        info!("Starting download threads");
        for task in track_tasks.iter() {
            let tx = tx.clone();
            let pb_clone = pb.clone();
            let t_clone = task.clone();
            pool.execute(move || {
                match yt_dlp::download_track(&t_clone.track_url) {
                    Ok(track_file) => tx
                        .send((t_clone, track_file))
                        .expect("channel is waiting for the pool"),
                    Err(e) => {
                        error!(
                            "Failed to fetch url ({}): {}",
                            &t_clone.track_url,
                            e.to_string()
                        );
                        eprintln!(
                            "{} {}",
                            WARNING,
                            style(format!(
                                "Failed to download track at {}",
                                &t_clone.track_url
                            ))
                            .yellow(),
                        );
                    }
                };
                pb_clone.inc(1);
            });
        }

        // Drop the last sender to stop `rx` waiting for message.
        drop(tx);

        info!("Processing thread results");
        while let Ok((task, file)) = rx.recv() {
            library
                .entries
                .entry(task.entry_name.to_owned())
                .and_modify(|e| {
                    e.tracks.push(Track {
                        url: task.track_url.to_owned(),
                        file,
                    })
                });
        }

        pb.finish_and_clear();
    }

    db::save(library)?;

    println!("{} Successfully synced library.", SUCCESS);
    Ok(())
}

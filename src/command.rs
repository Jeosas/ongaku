use console::{style, Emoji};
use indicatif::{MultiProgress, ProgressBar, ProgressIterator, ProgressStyle};
use log::{error, info};
use std::{
    num::NonZero,
    sync::mpsc::channel,
    thread::{self, available_parallelism},
    time::Duration,
};
use threadpool::ThreadPool;

use crate::{
    db::{self, entry::EntryType, Track},
    error::OngakuError,
    yt_dlp,
};

static SUCCESS: Emoji<'_, '_> = Emoji("✅", "");
static WARNING: Emoji<'_, '_> = Emoji("⚠️ ", "");
static INFO: Emoji<'_, '_> = Emoji(" ℹ️", "");

fn get_bar_style() -> ProgressStyle {
    ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:30.bold.cyan/.bold.dim} {pos:>7}/{len:7} ({eta})",
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
    db::init()
}

pub fn add(url: &str) -> Result<(), OngakuError> {
    info!("Running add command");
    let mut library = db::load()?;

    let new_entry = yt_dlp::get_url_info(url)?;

    info!("Checking entry duplication");
    if library.entries.contains_key(&new_entry.id) {
        return Err(OngakuError::AlreadyInLibrary(new_entry.name));
    }

    info!("Adding entry to library");
    library
        .entries
        .insert(new_entry.id.to_owned(), new_entry.clone());

    db::save(library)?;

    println!(
        "{} Successfully added {} to your library.",
        SUCCESS,
        style(&new_entry.name).cyan(),
    );
    println!(
        "{} Run {} to download tracks.",
        INFO,
        style("ongaku sync").cyan()
    );
    Ok(())
}

pub fn sync() -> Result<(), OngakuError> {
    #[derive(Debug, Clone)]
    struct Task {
        entry_id: String,
        entry_type: EntryType,
        entry_name: String,
        track_url: String,
    }

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
    let track_tasks = {
        let mut track_tasks: Vec<Task> = Vec::new();
        for (_, entry) in library
            .entries
            .iter()
            .progress_with_style(bar_style.clone())
        {
            let library_tracks: Vec<String> =
                entry.tracks.iter().map(|t| t.url.to_owned()).collect();

            match yt_dlp::get_tracks(&entry.url) {
                Ok(track_urls) => {
                    for track_url in track_urls {
                        if library_tracks.contains(&track_url) {
                            continue;
                        }
                        track_tasks.push(Task {
                            entry_id: entry.id.to_owned(),
                            entry_type: entry.r#type.try_into().expect("bounded by protobuf"),
                            entry_name: entry.name.to_owned(),
                            track_url,
                        })
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
        let pb = ProgressBar::new(track_tasks.len() as u64).with_style(bar_style.clone());

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
                match yt_dlp::download_track(
                    &t_clone.track_url,
                    &t_clone.entry_type,
                    &t_clone.entry_name,
                ) {
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
                .entry(task.entry_id.to_owned())
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

mod command;
mod db;
mod error;
mod yt_dlp;

use console::{style, Emoji};
use env_logger::Env;
use log::{error, info};
use std::{error::Error, process};

use clap::{Parser, Subcommand};

/// Manage your Youtube backed offline library.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize Ongaku in this directory
    Init,
    /// Add a new link to the library
    Add {
        /// Youtube URL (video, song, playlist, artist)
        url: String,
    },
    /// Sync the library
    Sync,

    /// Dump db in stdout
    #[cfg(debug_assertions)]
    Debug,
}

fn main() {
    env_logger::Builder::from_env(Env::default().filter_or("ONGAKU_LOG", "off")).init();

    info!("Parsing args");
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => command::init(),
        Commands::Add { url } => command::add(url),
        Commands::Sync => command::sync(),

        #[cfg(debug_assertions)]
        Commands::Debug => {
            dbg!(db::load().expect("failed to load db"));
            Ok(())
        }
    }
    .unwrap_or_else(|e| {
        eprintln!("{} {}", Emoji("💥", ""), style(e.to_string()).bold().red());
        if let Some(source) = e.source() {
            error!("{}", source.to_string());
        }
        process::exit(1);
    });
}

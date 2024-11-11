mod command;
mod db;
mod error;

use console::{style, Emoji};
use std::process;

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
        /// Entry name
        name: String,
        /// Youtube URL (video, song, playlist, artist)
        url: String,
    },
    /// Sync the library
    Sync {
        /// Verify library integrity before syncing
        #[arg(short, long)]
        verify: bool,
    },
    /// Verify library integrity
    Verify,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => command::init(),
        Commands::Add { name, url } => command::add(name, url),
        Commands::Sync { verify } => command::sync(verify.to_owned()),
        Commands::Verify => command::verify(false),
    }
    .unwrap_or_else(|e| {
        eprintln!("{} {}", Emoji("💥", ""), style(e.to_string()).bold().red());
        process::exit(1);
    });
}

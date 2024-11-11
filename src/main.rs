mod command;
mod error;

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

fn main() -> Result<(), error::OngakuError> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { name, url } => command::add(name, url),
        Commands::Sync { verify } => command::sync(verify.to_owned()),
        Commands::Verify => command::verify(false),
    }
}

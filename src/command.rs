use console::{style, Emoji};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::{thread, time::Duration};

use crate::{db, error::OngakuError};

static SUCCESS: Emoji<'_, '_> = Emoji("✅", "");
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
    db::init()
}

pub fn add(name: &str, url: &str) -> Result<(), OngakuError> {
    println!("TODO in add command");
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

pub fn sync(verify_: bool) -> Result<(), OngakuError> {
    if verify_ {
        verify(true)?;
    };

    let bar_style = get_bar_style();
    let spinner_style = get_spinner_style();

    println!(
        "{} {} Reading database",
        style("[1/3]").bold().dim(),
        Emoji("💽", "")
    );
    let artists: Vec<i32> = {
        let pb = ProgressBar::new(10)
            .with_style(spinner_style.clone())
            .with_message("Reading database");
        pb.enable_steady_tick(Duration::from_millis(80));

        thread::sleep(Duration::from_secs(2));

        pb.finish_and_clear();

        (0..12).collect()
    };

    println!(
        "{} {} Listing missing tracks",
        style("[2/3]").bold().dim(),
        Emoji("🔍", "")
    );
    let tracks: Vec<i32> = {
        let pb = ProgressBar::new(artists.len() as u64).with_style(bar_style.clone());

        for _ in artists.iter() {
            thread::sleep(Duration::from_millis(80));
            pb.inc(1);
        }

        pb.finish_and_clear();

        (0..123).collect()
    };

    println!(
        "{} {} Downloading missing tracks",
        style("[3/3]").bold().dim(),
        Emoji("📥", "")
    );
    {
        let pb = ProgressBar::new(tracks.len() as u64).with_style(bar_style.clone());

        for _ in tracks.iter() {
            thread::sleep(Duration::from_millis(10));
            pb.inc(1);
        }

        pb.finish_and_clear();
    }

    println!("{} Successfully synced library.", SUCCESS);
    Ok(())
}

pub fn verify(from_sync: bool) -> Result<(), OngakuError> {
    println!("{} Verifying library intergrity", Emoji("🔄", ""));

    {
        let m = MultiProgress::new();
        let pb = m.add(ProgressBar::new(123).with_style(get_bar_style()));

        for n in 0..123 {
            thread::sleep(Duration::from_millis(100));
            if n % 50 == 0 {
                m.println(format!("{} Coudn't find file '{}'.", Emoji("❓", ""), n))
                    .expect("failed to print log line");
            }
            pb.inc(1);
        }

        pb.finish_and_clear();
        m.clear().expect("failed to clear MultiProgressBar");
    };

    println!("{} Verification finished.", SUCCESS);
    if !from_sync {
        println!(
            "{} Found missing files. Run {} to re-download them.",
            INFO,
            style("ongaku sync").cyan()
        );
    }
    Ok(())
}

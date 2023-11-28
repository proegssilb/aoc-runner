use std::{io::BufRead, process::Command};

use anyhow::{Ok, Context};
use thiserror::Error;

use crate::{
    cli::Cli,
    iodomain::{credentials::{ConfigFileCookieStore, CookieStore}, cargo::WorkspaceMeta},
};

const AUTH_MESSAGE: &str = "This command doesn't implement proper authenticaion yet. Use your browser to visit and log in to the AOC website, then copy the value of the 'session' cookie, and paste it here: ";

pub fn login<T: BufRead>(readfn: fn() -> T, _cli: Cli) -> anyhow::Result<()> {
    let mut store = ConfigFileCookieStore::new()?;
    let mut in_stream = readfn();
    print!("{}", AUTH_MESSAGE);

    let mut cookie: String = String::new();
    in_stream.read_line(&mut cookie)?;

    store.set_session_cookie(cookie.trim())?;

    println!("\nSession cookie stored successfully.");
    Ok(())
}

pub fn input<T: BufRead>(_readfn: fn() -> T, _cli: Cli) -> anyhow::Result<()> {
    todo!()
}

pub fn prepare<T: BufRead>(_readfn: fn() -> T, _cli: Cli) -> anyhow::Result<()> {
    todo!()
}

#[derive(Error, Debug)]
enum RunError {
    #[error("Not currently in a AOC Year's crate. Failed to detect currently selected year.")]
    NotInACrate,
    #[error("No targets found. Are there binaries in your Cargo.toml named similar to `day15`?")]
    NoTargetsFound,
    #[error("Could not pick out a default year. Are you currently in a year-specific crate's folder?")]
    NoYearsFound,
    #[error("Could not find year specified. Is that year a crate in your workspace?")]
    YearNotFound,
}

pub fn run<T: BufRead>(_readfn: fn() -> T, cli: Cli) -> anyhow::Result<()> {

    // Get some data together
    let data = WorkspaceMeta::load()
        .context("Failed to load data for the current cargo workspace. Are you in a crate or workspace?")?;

    for pack_id in data.worspace_data.workspace_members.iter() {
        println!("  Pack ID: {}", pack_id);
        if let Some(pack) = data.package_map().get(&pack_id) {
            println!("    Manifest path: {}", pack.manifest_path);
            for t in pack.targets.iter() {
                println!("    Target: {} ({})", t.name, t.src_path);
            }
        }
    }

    // Figure out which year we're in
    let Some(pack) = (match cli.year {
        None => data.current_package(),
        Some(y) => data.get_year_map().get(&y).map(|&p| p),
    }) else {
        return Err(RunError::YearNotFound.into());
    };

    let Some(curr_package) = data.current_package() else {
        return Err(RunError::NotInACrate.into())
    };

    // Figure out the selected day
    let Some(&ref target) = (match cli.day {
        None => data.get_target_for_latest_day(pack),
        Some(d) => data.get_day_map(pack).get(&d).map(|&p| p),
    }) else {
        return Err(RunError::NoTargetsFound.into())
    };

    // And now, to run the target!
    println!("Running solutions for {}", target.name);

    let mut child = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg(&target.name)
        .current_dir(pack.manifest_path.parent().unwrap())
        .spawn()?;

    child.wait()?;

    Ok(())
}

pub fn benchmark<T: BufRead>(_readfn: fn() -> T, _cli: Cli) -> anyhow::Result<()> {
    todo!()
}

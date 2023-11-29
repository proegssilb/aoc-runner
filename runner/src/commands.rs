use std::{
    cmp::min,
    fs::{create_dir_all, write},
    io::BufRead,
    process::Command,
    sync::Arc,
};

use anyhow::{anyhow, Context, Ok};
use chrono::{Datelike, Utc};
use chrono_tz::US::Eastern;
use regex::Regex;
use reqwest::{blocking::ClientBuilder, cookie::Jar, Url};
use thiserror::Error;

use crate::{
    cli::Cli,
    iodomain::{
        cargo::WorkspaceMeta,
        credentials::{ConfigFileCookieStore, CookieStore},
    },
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

pub fn input<T: BufRead>(readfn: fn() -> T, cli: Cli) -> anyhow::Result<()> {
    let store = ConfigFileCookieStore::new()?;
    let stored_session = store.get_session_cookie()?;
    if stored_session == "" {
        login(readfn, cli.clone())?;
    }

    let meta = WorkspaceMeta::load()?;
    let session = store.get_session_cookie()?;

    //URL: https://adventofcode.com/2022/day/22/input

    let jar = Jar::default();
    let cookie = format!("session={}", session);
    let url = "https://adventofcode.com".parse::<Url>().unwrap();
    jar.add_cookie_str(&cookie, &url);
    let store = Arc::new(jar);

    let cb = ClientBuilder::new();
    let client = cb.cookie_provider(store).build()?;

    // Get the current year.
    let mut year = match cli.year {
        Some(y) => y,
        None => {
            let year_filter = Regex::new(r"(\d{4})$").unwrap();
            let current_package = meta
                .current_package()
                .ok_or(anyhow!("Not in a package. Please specify a year or `cd`."))?;
            let matches = year_filter.captures(&current_package.name).unwrap();
            let year_text = matches.get(1).unwrap().as_str();

            year_text.parse::<u16>()?
        }
    };

    // Get the current day.

    let day = match &cli.day {
        Some(d) => *d,
        None => {
            let date_utc = Utc::now();
            let date_est = date_utc.with_timezone(&Eastern);

            if date_est.month() == 12 {
                min(date_est.day() as u8, 25u8)
            } else {
                // Current date is the 25th of the previous year.
                year = year - 1;
                25u8
            }
        }
    };

    let input_url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let response = client.get(input_url).send()?;

    let mut target_name = meta.worspace_data.workspace_root.clone();
    target_name.push("input");
    target_name.push(year.to_string());
    create_dir_all(&target_name)?;
    target_name.push(format!("{}.txt", day));

    write(target_name, response.text()?)?;

    Ok(())
}

pub fn prepare<T: BufRead>(_readfn: fn() -> T, _cli: Cli) -> anyhow::Result<()> {
    todo!()
}

#[derive(Error, Debug)]
enum RunError {
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

    // for pack_id in data.worspace_data.workspace_members.iter() {
    //     println!("  Pack ID: {}", pack_id);
    //     if let Some(pack) = data.package_map().get(&pack_id) {
    //         println!("    Manifest path: {}", pack.manifest_path);
    //         for t in pack.targets.iter() {
    //             println!("    Target: {} ({})", t.name, t.src_path);
    //         }
    //     }
    // }

    // Figure out which year we're in
    let pack = match cli.year {
        None => data.current_package().ok_or(RunError::NoYearsFound),
        Some(y) => data.get_year_map().get(&y).map(|&p| p).ok_or(RunError::YearNotFound),
    }?;

    // Figure out the selected day
    let Some(&ref target) = (match cli.day {
        None => data.get_target_for_latest_day(pack),
        Some(d) => data.get_day_map(pack).get(&d).map(|&p| p),
    }) else {
        return Err(RunError::NoTargetsFound.into());
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

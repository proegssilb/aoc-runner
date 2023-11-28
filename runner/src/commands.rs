use std::{collections::HashMap, env, io::BufRead, process::Command};

use anyhow::Ok;
use cargo_metadata::{Package, PackageId, Target};
use regex::Regex;
use thiserror::Error;

use crate::{
    cli::Cli,
    iodomain::credentials::{ConfigFileCookieStore, CookieStore},
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
    NoTargetsFound
}

pub fn run<T: BufRead>(_readfn: fn() -> T, _cli: Cli) -> anyhow::Result<()> {
    let day_filter: Regex = Regex::new(r"^d(?:ay)?(\d{1,2})$").unwrap();

    // Get some data together
    let cmd = cargo_metadata::MetadataCommand::new();
    let meta = cmd.exec()?;
    let curr_dir = env::current_dir()?;
    println!("Current directory: {}", env::current_dir()?.display());
    println!("Workspace root: {}", meta.workspace_root);
    let _package_map: HashMap<&PackageId, &Package> = HashMap::from_iter(meta.packages.iter().map(|p| (&p.id, p)));

    // for pack_id in meta.workspace_members {
    //     println!("  Pack ID: {}", pack_id);
    //     if let Some(pack) = package_map.get(&pack_id) {
    //         println!("    Manifest path: {}", pack.manifest_path);
    //         for t in pack.targets.iter() {
    //             println!("    Target: {} ({})", t.name, t.src_path);
    //         }
    //     }
    // }

    // Figure out which year we're in
    let &curr_package = meta
        .workspace_packages()
        .iter()
        .filter(|p| curr_dir.starts_with(p.manifest_path.parent().unwrap()))
        .next()
        .ok_or(RunError::NotInACrate)?;

    // Figure out the latest day
    let mut targets: Vec<(&Target, u8)> = Vec::new();

    for target in curr_package.targets.iter() {
        let Some(captures) = day_filter.captures(&target.name) else { continue; };
        let Some(m) = captures.get(1) else {
            println!("Matched without finding a capture group: {}", target.name);
            continue;
        };
        let day_num: Result<u8, std::num::ParseIntError> = m.as_str().parse();
        match day_num {
            Result::Err(e) => { println!("Failed to parse num: {}, '{}' ({})", target.name, m.as_str(), e.to_string())},
            Result::Ok(dn) => { targets.push((target, dn)) },
        }
    }

    targets.sort_by(|a, b| Ord::cmp(&b.1, &a.1));

    if targets.len() == 0 {
        println!("No targets found.");
        return Err(RunError::NoTargetsFound.into());
    }

    let &target = &targets[0].0;

    // And now, to run the target!
    println!("Running solutions for {}", target.name);

    let output = Command::new("cargo").arg("run").arg("--bin").arg(&target.name).output()?;

    println!("{}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}

pub fn benchmark<T: BufRead>(_readfn: fn() -> T, _cli: Cli) -> anyhow::Result<()> {
    todo!()
}

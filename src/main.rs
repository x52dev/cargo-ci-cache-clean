//! Clean up unnecessary Cargo artifacts to improve CI caching performance.

#![deny(rust_2018_idioms, nonstandard_style)]
#![warn(future_incompatible)]

use std::{fs, path::Path, process};

use walkdir::WalkDir;

fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let cargo_home = match home::cargo_home() {
        Ok(cargo_home) => cargo_home,
        Err(err) => {
            tracing::error!("Cargo's home path could not be determined: {err}");
            process::exit(1);
        }
    };

    if !cargo_home.is_dir() {
        tracing::error!(
            "Cargo's home path is not a directory: {}",
            cargo_home.display()
        );
        process::exit(1);
    }

    let registry_sources = cargo_home.join("registry").join("src");
    let git_checkouts = cargo_home.join("git").join("checkouts");

    for dir in &[&registry_sources, &git_checkouts] {
        if dir.is_dir() {
            let size = dir_size(dir);
            tracing::info!("Removing {:.2}MB from {}", size as f64 / 1e7, dir.display());

            remove_file(dir);
        }
    }
}

fn dir_size(path: &Path) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| fs::metadata(entry.path()).ok())
        .map(|md| md.len())
        .sum()
}

fn remove_file(path: &Path) {
    if path.is_file() && fs::remove_file(path).is_err() {
        tracing::warn!("failed to remove file: {}", path.display());
    } else if path.is_dir() && fs::remove_dir_all(path).is_err() {
        tracing::warn!("failed to recursively remove directory: {}", path.display());
    } else if path.is_symlink() {
        tracing::debug!("Not removing symlink: {}", path.display());
    }
}

#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};

use crate::domain::settings::Settings;

pub fn load() -> Settings {
    load_from(&file_path())
}

pub fn save(settings: &Settings) -> anyhow::Result<()> {
    save_to(&file_path(), settings)
}

fn file_path() -> PathBuf {
    std::env::var_os("APPDATA")
        .map(PathBuf::from)
        .unwrap_or_default()
        .join("CopyAnywhere")
        .join("settings.json")
}

fn load_from(path: &Path) -> Settings {
    fs::read_to_string(path)
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_default()
}

fn save_to(path: &Path, settings: &Settings) -> anyhow::Result<()> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    fs::write(path, serde_json::to_string_pretty(settings)?)?;
    Ok(())
}

#[cfg(test)]
mod tests;

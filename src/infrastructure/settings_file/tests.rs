use std::fs;
use std::path::PathBuf;

use super::{load_from, save_to};
use crate::domain::settings::Settings;

fn scratch_dir(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("copyanywhere-{}-{name}", std::process::id()))
}

#[test]
fn missing_file_loads_default() {
    let dir = scratch_dir("missing");
    assert_eq!(load_from(&dir.join("settings.json")), Settings::default());
}

#[test]
fn corrupt_file_loads_default() {
    let dir = scratch_dir("corrupt");
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join("settings.json");
    fs::write(&path, "{not json").unwrap();
    assert_eq!(load_from(&path), Settings::default());
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn saved_settings_round_trip_through_nested_dir() {
    let dir = scratch_dir("round_trip");
    let path = dir.join("nested").join("settings.json");
    let mut settings = Settings {
        sound_enabled: false,
        ..Settings::default()
    };
    settings.excluded.add("notepad.exe");
    save_to(&path, &settings).unwrap();
    assert_eq!(load_from(&path), settings);
    fs::remove_dir_all(&dir).unwrap();
}

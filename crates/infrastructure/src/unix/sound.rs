use std::path::PathBuf;
use std::process::Command;
use std::sync::OnceLock;

use crate::tone::tone_wav;

#[cfg(target_os = "macos")]
const PLAYERS: [&[&str]; 1] = [&["afplay"]];
#[cfg(not(target_os = "macos"))]
const PLAYERS: [&[&str]; 2] = [&["paplay"], &["aplay", "-q"]];

static TONE: OnceLock<Option<PathBuf>> = OnceLock::new();

pub fn play_copied() {
    let Some(tone) = TONE.get_or_init(write_tone) else {
        return;
    };
    let started = PLAYERS.iter().find_map(|player| {
        Command::new(player[0])
            .args(&player[1..])
            .arg(tone)
            .spawn()
            .ok()
    });
    // reaps the player once it is done, so no zombie is left behind
    if let Some(mut child) = started {
        std::thread::spawn(move || child.wait());
    }
}

fn write_tone() -> Option<PathBuf> {
    let path = std::env::temp_dir().join("copyanywhere-copied.wav");
    std::fs::write(&path, tone_wav()).ok()?;
    Some(path)
}

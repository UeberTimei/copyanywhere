use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::settings_file;

#[cfg(test)]
mod tests;

pub fn set(enabled: bool) -> Result<()> {
    set_at(&entry_path(), enabled, &entry(&std::env::current_exe()?))
}

pub fn is_enabled() -> bool {
    entry_path().exists()
}

#[cfg(target_os = "macos")]
fn entry_path() -> PathBuf {
    settings_file::home().join("Library/LaunchAgents/com.copyanywhere.plist")
}

#[cfg(not(target_os = "macos"))]
fn entry_path() -> PathBuf {
    settings_file::config_dir().join("autostart/copyanywhere.desktop")
}

#[cfg(target_os = "macos")]
fn entry(exe: &Path) -> String {
    let exe = exe
        .to_string_lossy()
        .replace('&', "&amp;")
        .replace('<', "&lt;");
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>Label</key><string>com.copyanywhere</string>
<key>ProgramArguments</key><array><string>{exe}</string></array>
<key>RunAtLoad</key><true/>
</dict></plist>
"#
    )
}

#[cfg(not(target_os = "macos"))]
fn entry(exe: &Path) -> String {
    format!(
        "[Desktop Entry]\nType=Application\nName=CopyAnywhere\nExec=\"{}\"\n",
        exe.display()
    )
}

fn set_at(path: &Path, enabled: bool, contents: &str) -> Result<()> {
    if enabled {
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        fs::write(path, contents)?;
    } else {
        ignore_missing(fs::remove_file(path))?;
    }
    Ok(())
}

fn ignore_missing(result: io::Result<()>) -> io::Result<()> {
    match result {
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        other => other,
    }
}

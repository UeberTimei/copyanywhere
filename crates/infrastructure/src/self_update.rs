use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Result, bail};
use domain::update::Release;
use sha2::{Digest, Sha256};

use crate::http;

/// Replaces the running exe with the release's and starts it; the caller must exit afterwards.
pub fn install(release: &Release) -> Result<()> {
    let exe = std::env::current_exe()?;
    let Some(checksum_url) = &release.checksum_url else {
        bail!("this release has no checksum to verify the download");
    };
    let bytes = http::get_bytes(&release.exe_url)?;
    if !matches(&bytes, &http::get_text(checksum_url)?) {
        bail!("the download does not match its checksum");
    }
    let (fresh, old) = (beside(&exe, "new"), beside(&exe, "old"));
    fs::write(&fresh, bytes)?;
    // Windows lets a running exe be renamed, never overwritten.
    fs::remove_file(&old).ok();
    fs::rename(&exe, &old)?;
    if let Err(error) = fs::rename(&fresh, &exe) {
        fs::rename(&old, &exe).ok();
        return Err(error.into());
    }
    Command::new(&exe).spawn()?;
    Ok(())
}

/// Deletes the exe an earlier update left behind.
pub fn remove_leftover() {
    if let Ok(exe) = std::env::current_exe() {
        fs::remove_file(beside(&exe, "old")).ok();
    }
}

fn beside(exe: &Path, extension: &str) -> PathBuf {
    exe.with_extension(format!("exe.{extension}"))
}

/// `checksum` is `sha256sum` output: the hex digest, then the file name.
fn matches(bytes: &[u8], checksum: &str) -> bool {
    let digest: String = Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    checksum
        .split_whitespace()
        .next()
        .is_some_and(|expected| expected.eq_ignore_ascii_case(&digest))
}

#[cfg(test)]
mod tests;

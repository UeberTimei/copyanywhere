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
    make_executable(&fresh)?;
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

#[cfg(unix)]
fn make_executable(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    Ok(fs::set_permissions(
        path,
        fs::Permissions::from_mode(0o755),
    )?)
}

#[cfg(not(unix))]
fn make_executable(_: &Path) -> Result<()> {
    Ok(())
}

/// Deletes the exe an earlier update left behind.
pub fn remove_leftover() {
    if let Ok(exe) = std::env::current_exe() {
        fs::remove_file(beside(&exe, "old")).ok();
    }
}

fn beside(exe: &Path, suffix: &str) -> PathBuf {
    let mut name = exe.file_name().unwrap_or_default().to_owned();
    name.push(format!(".{suffix}"));
    exe.with_file_name(name)
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

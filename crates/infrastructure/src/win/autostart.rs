use std::io;

use anyhow::Result;
use winreg::RegKey;
use winreg::enums::{HKEY_CURRENT_USER, KEY_READ};

const RUN_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
const VALUE_NAME: &str = "CopyAnywhere";

pub fn set(enabled: bool) -> Result<()> {
    let command = format!("\"{}\"", std::env::current_exe()?.display());
    set_in(RUN_KEY, enabled, &command)
}

pub fn is_enabled() -> bool {
    is_enabled_in(RUN_KEY)
}

fn set_in(subkey: &str, enabled: bool, command: &str) -> Result<()> {
    let (key, _) = RegKey::predef(HKEY_CURRENT_USER).create_subkey(subkey)?;
    if enabled {
        key.set_value(VALUE_NAME, &command)?;
    } else {
        ignore_missing(key.delete_value(VALUE_NAME))?;
    }
    Ok(())
}

fn is_enabled_in(subkey: &str) -> bool {
    RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey_with_flags(subkey, KEY_READ)
        .and_then(|key| key.get_value::<String, _>(VALUE_NAME))
        .is_ok()
}

fn ignore_missing(result: io::Result<()>) -> io::Result<()> {
    match result {
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        other => other,
    }
}

#[cfg(test)]
mod tests;

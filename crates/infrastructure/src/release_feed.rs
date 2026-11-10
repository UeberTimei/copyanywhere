use anyhow::{Result, anyhow};
use domain::update::Release;
use serde_json::Value;

const LATEST: &str = "https://api.github.com/repos/UeberTimei/copyanywhere/releases/latest";
// Release assets are named per architecture; keep in sync with the release job in ci.yml.
#[cfg(target_arch = "x86_64")]
const ARCH: &str = "x64";
#[cfg(target_arch = "x86")]
const ARCH: &str = "x86";
#[cfg(target_arch = "aarch64")]
const ARCH: &str = "arm64";

pub fn latest() -> Result<Release> {
    let body = crate::http::get_text(LATEST)?;
    parse(&serde_json::from_str(&body)?).ok_or_else(|| anyhow!("unexpected release payload"))
}

fn parse(release: &Value) -> Option<Release> {
    let exe_asset = format!("CopyAnywhere-{ARCH}.exe");
    Some(Release {
        version: release["tag_name"].as_str()?.to_owned(),
        notes: release["body"]
            .as_str()
            .unwrap_or_default()
            .trim()
            .to_owned(),
        exe_url: asset_url(release, &exe_asset)?,
        checksum_url: asset_url(release, &format!("{exe_asset}.sha256")),
    })
}

fn asset_url(release: &Value, name: &str) -> Option<String> {
    release["assets"]
        .as_array()?
        .iter()
        .find(|asset| asset["name"] == name)
        .and_then(|asset| asset["browser_download_url"].as_str())
        .filter(|url| url.starts_with("https://"))
        .map(str::to_owned)
}

#[cfg(test)]
mod tests;

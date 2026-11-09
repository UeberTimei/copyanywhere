use anyhow::{Result, anyhow};
use domain::update::Release;
use serde_json::Value;

const LATEST: &str = "https://api.github.com/repos/UeberTimei/copyanywhere/releases/latest";
const EXE_ASSET: &str = "CopyAnywhere.exe";
const CHECKSUM_ASSET: &str = "CopyAnywhere.exe.sha256";

pub fn latest() -> Result<Release> {
    let body = crate::http::get_text(LATEST)?;
    parse(&serde_json::from_str(&body)?).ok_or_else(|| anyhow!("unexpected release payload"))
}

fn parse(release: &Value) -> Option<Release> {
    Some(Release {
        version: release["tag_name"].as_str()?.to_owned(),
        notes: release["body"]
            .as_str()
            .unwrap_or_default()
            .trim()
            .to_owned(),
        exe_url: asset_url(release, EXE_ASSET)?,
        checksum_url: asset_url(release, CHECKSUM_ASSET)?,
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

use std::time::Duration;

use anyhow::Result;

const TIMEOUT: Duration = Duration::from_secs(30);
const EXE_LIMIT: u64 = 100 * 1024 * 1024;

pub fn get_text(url: &str) -> Result<String> {
    Ok(request(url)?.body_mut().read_to_string()?)
}

pub fn get_bytes(url: &str) -> Result<Vec<u8>> {
    Ok(request(url)?
        .body_mut()
        .with_config()
        .limit(EXE_LIMIT)
        .read_to_vec()?)
}

fn request(url: &str) -> Result<ureq::http::Response<ureq::Body>> {
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .timeout_global(Some(TIMEOUT))
        .build()
        .into();
    Ok(agent.get(url).header("User-Agent", "CopyAnywhere").call()?)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Release {
    pub version: String,
    pub notes: String,
    pub exe_url: String,
    pub checksum_url: Option<String>,
}

/// Whether the user should be offered `release`: it is newer than `current` and not skipped.
pub fn is_offered(release: &Release, current: &str, skipped: &str) -> bool {
    release.version != skipped && is_newer(&release.version, current)
}

fn is_newer(latest: &str, current: &str) -> bool {
    match (parse(latest), parse(current)) {
        (Some(latest), Some(current)) => latest > current,
        _ => false,
    }
}

fn parse(version: &str) -> Option<Vec<u64>> {
    version
        .trim_start_matches('v')
        .split('.')
        .map(|part| part.parse().ok())
        .collect()
}

#[cfg(test)]
mod tests;

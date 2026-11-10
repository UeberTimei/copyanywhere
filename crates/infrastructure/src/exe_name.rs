#[cfg(test)]
mod tests;

pub fn exe_name(path: &str) -> String {
    path.rsplit(['\\', '/'])
        .next()
        .unwrap_or_default()
        .to_lowercase()
}

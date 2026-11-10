use crate::cargo;
use std::fs;

const MANIFEST: &str = "crates/app/Cargo.toml";

/// Sets the app version in its manifest and refreshes Cargo.lock; the release workflow tags it.
pub fn run(version: Option<&str>) -> bool {
    let Some(version) = version.filter(|v| is_semver(v)) else {
        eprintln!("usage: cargo xtask bump <major.minor.patch>");
        return false;
    };
    let Ok(manifest) = fs::read_to_string(MANIFEST) else {
        eprintln!("cannot read {MANIFEST}");
        return false;
    };
    let Some(updated) = replace_version(&manifest, version) else {
        eprintln!("no version line in {MANIFEST}");
        return false;
    };
    fs::write(MANIFEST, updated).is_ok() && cargo::run(&["update", "--workspace", "--offline"])
}

fn is_semver(text: &str) -> bool {
    let parts: Vec<_> = text.split('.').collect();
    parts.len() == 3
        && parts
            .iter()
            .all(|part| !part.is_empty() && part.bytes().all(|b| b.is_ascii_digit()))
}

fn replace_version(manifest: &str, version: &str) -> Option<String> {
    let mut replaced = false;
    let lines: Vec<String> = manifest
        .lines()
        .map(|line| {
            if !replaced && line.starts_with("version = ") {
                replaced = true;
                format!("version = \"{version}\"")
            } else {
                line.to_string()
            }
        })
        .collect();
    replaced.then(|| lines.join("\n") + "\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replaces_only_the_package_version() {
        let toml = "[package]\nversion = \"1.0.0\"\n[dependencies]\nx = { version = \"1\" }\n";
        let out = replace_version(toml, "1.1.0").unwrap();
        assert_eq!(
            out,
            "[package]\nversion = \"1.1.0\"\n[dependencies]\nx = { version = \"1\" }\n"
        );
    }

    #[test]
    fn rejects_non_semver() {
        assert!(is_semver("1.2.3"));
        assert!(!is_semver("1.2"));
        assert!(!is_semver("v1.2.3"));
    }
}

use super::{Release, is_offered};

fn release(version: &str) -> Release {
    Release {
        version: version.into(),
        notes: String::new(),
        exe_url: String::new(),
        checksum_url: String::new(),
    }
}

#[test]
fn newer_release_is_offered() {
    assert!(is_offered(&release("v1.0.1"), "1.0.0", ""));
    assert!(is_offered(&release("v1.10.0"), "1.9.0", ""));
}

#[test]
fn same_or_older_release_is_not_offered() {
    assert!(!is_offered(&release("v1.0.0"), "1.0.0", ""));
    assert!(!is_offered(&release("v0.9.9"), "1.0.0", ""));
}

#[test]
fn skipped_release_is_not_offered() {
    assert!(!is_offered(&release("v2.0.0"), "1.0.0", "v2.0.0"));
}

#[test]
fn unparsable_version_is_not_offered() {
    assert!(!is_offered(&release("nightly"), "1.0.0", ""));
}

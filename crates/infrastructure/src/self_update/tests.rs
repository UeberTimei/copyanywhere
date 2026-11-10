use std::path::Path;

use super::{beside, matches};

const EMPTY_SHA256: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

#[test]
fn checksum_accepts_the_matching_digest_with_a_file_name() {
    assert!(matches(b"", &format!("{EMPTY_SHA256}  CopyAnywhere.exe\n")));
}

#[test]
fn checksum_rejects_other_content_and_empty_text() {
    assert!(!matches(b"tampered", EMPTY_SHA256));
    assert!(!matches(b"", ""));
}

#[test]
fn leftover_files_sit_next_to_the_exe() {
    let exe = Path::new(r"C:\apps\CopyAnywhere.exe");
    assert_eq!(
        beside(exe, "old"),
        Path::new(r"C:\apps\CopyAnywhere.exe.old")
    );
}

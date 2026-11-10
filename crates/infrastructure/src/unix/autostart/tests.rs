use std::path::Path;

use super::{entry, set_at};

#[test]
fn enable_disable_round_trip_and_disable_when_absent() {
    let path = std::env::temp_dir().join("copyanywhere-autostart-test/entry");
    set_at(&path, false, "x").unwrap();
    assert!(!path.exists());
    set_at(&path, true, "x").unwrap();
    assert!(path.exists());
    set_at(&path, false, "x").unwrap();
    assert!(!path.exists());
}

#[test]
fn entry_launches_the_exe() {
    assert!(entry(Path::new("/opt/copyanywhere")).contains("/opt/copyanywhere"));
}

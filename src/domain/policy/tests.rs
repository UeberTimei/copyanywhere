use super::should_copy;
use crate::domain::exclusions::ExcludedPrograms;

fn excluding(name: &str) -> ExcludedPrograms {
    let mut excluded = ExcludedPrograms::default();
    excluded.add(name);
    excluded
}

#[test]
fn disabled_blocks_copy() {
    assert!(!should_copy(
        false,
        Some("notepad.exe"),
        &ExcludedPrograms::default()
    ));
}

#[test]
fn enabled_without_program_allows_copy() {
    assert!(should_copy(true, None, &ExcludedPrograms::default()));
}

#[test]
fn excluded_program_blocks_copy_ignoring_case() {
    assert!(!should_copy(
        true,
        Some("Explorer.EXE"),
        &excluding("explorer.exe")
    ));
}

#[test]
fn non_excluded_program_allows_copy() {
    assert!(should_copy(
        true,
        Some("notepad.exe"),
        &excluding("explorer.exe")
    ));
}

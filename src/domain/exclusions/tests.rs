use super::ExcludedPrograms;

#[test]
fn add_normalizes_case_and_whitespace() {
    let mut list = ExcludedPrograms::default();
    assert!(list.add("  Explorer.EXE  "));
    assert_eq!(list.iter().collect::<Vec<_>>(), ["explorer.exe"]);
}

#[test]
fn add_rejects_duplicate() {
    let mut list = ExcludedPrograms::default();
    assert!(list.add("cmd.exe"));
    assert!(!list.add("CMD.exe"));
    assert_eq!(list.iter().count(), 1);
}

#[test]
fn add_rejects_empty() {
    let mut list = ExcludedPrograms::default();
    assert!(!list.add(""));
    assert!(!list.add("   "));
    assert_eq!(list.iter().count(), 0);
}

#[test]
fn remove_is_case_insensitive() {
    let mut list = ExcludedPrograms::default();
    list.add("putty.exe");
    list.remove("PuTTY.exe");
    assert!(!list.contains("putty.exe"));
}

#[test]
fn contains_is_case_insensitive() {
    let mut list = ExcludedPrograms::default();
    list.add("wt.exe");
    assert!(list.contains("WT.EXE"));
    assert!(!list.contains("cmd.exe"));
}

#[test]
fn serializes_as_plain_array() {
    let mut list = ExcludedPrograms::default();
    list.add("explorer.exe");
    let json = serde_json::to_string(&list).expect("serialize");
    assert_eq!(json, r#"["explorer.exe"]"#);
}

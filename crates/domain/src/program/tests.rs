use super::Program;

#[test]
fn label_joins_title_and_exe_with_a_dash() {
    let program = Program {
        exe: "chrome.exe".into(),
        title: Some("Google Chrome".into()),
    };
    assert_eq!(program.label(), "Google Chrome - chrome.exe");
}

#[test]
fn label_is_the_exe_without_a_title() {
    assert_eq!(Program::bare("zed.exe").label(), "zed.exe");
}

#[test]
fn label_skips_a_title_that_repeats_the_exe() {
    let program = Program {
        exe: "zed.exe".into(),
        title: Some("Zed.exe".into()),
    };
    assert_eq!(program.label(), "zed.exe");
}

use domain::exclusions::ExcludedPrograms;
use domain::program::Program;
use domain::settings::Settings;

use super::{Feature, apply_toggle, is_on, pickable_programs};

#[test]
fn apply_toggle_flips_each_feature() {
    let mut settings = Settings::default();
    apply_toggle(&mut settings, Feature::LaunchAtLogin);
    assert!(is_on(&settings, Feature::LaunchAtLogin));
    apply_toggle(&mut settings, Feature::Sound);
    assert!(!is_on(&settings, Feature::Sound));
    apply_toggle(&mut settings, Feature::Notification);
    assert!(!is_on(&settings, Feature::Notification));
}

#[test]
fn pickable_skips_excluded_case_insensitively() {
    let mut excluded = ExcludedPrograms::default();
    excluded.add("Code.exe");
    let picks = pickable_programs(
        vec![Program::bare("CODE.exe"), Program::bare("notepad.exe")],
        &excluded,
    );
    assert_eq!(picks, [Program::bare("notepad.exe")]);
}

#[test]
fn pickable_drops_blank_and_duplicate_names() {
    let excluded = ExcludedPrograms::default();
    let running = vec![
        Program::bare("  "),
        Program::bare("zed.EXE"),
        Program::bare("Zed.exe"),
        Program::bare("Alpha.exe"),
    ];
    let picks = pickable_programs(running, &excluded);
    assert_eq!(picks.len(), 2);
    assert_eq!(picks[0].exe, "Alpha.exe");
}

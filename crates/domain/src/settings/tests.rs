use super::Settings;

#[test]
fn default_enables_sound_and_toast_but_not_login() {
    let settings = Settings::default();
    assert!(!settings.launch_at_login);
    assert!(settings.sound_enabled);
    assert!(settings.toast_enabled);
}

#[test]
fn default_excludes_explorer() {
    assert!(Settings::default().excluded.contains("explorer.exe"));
}

#[test]
fn json_round_trip_preserves_settings() {
    let settings = Settings::default();
    let json = serde_json::to_string(&settings).unwrap();
    let restored: Settings = serde_json::from_str(&json).unwrap();
    assert_eq!(restored, settings);
}

#[test]
fn empty_json_deserializes_to_default() {
    let settings: Settings = serde_json::from_str("{}").unwrap();
    assert_eq!(settings, Settings::default());
}

#[test]
fn default_checks_for_updates_and_skips_nothing() {
    let settings = Settings::default();
    assert!(settings.check_updates);
    assert!(settings.skipped_version.is_empty());
}

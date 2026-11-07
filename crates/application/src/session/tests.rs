use domain::settings::Settings;

use super::Session;

#[test]
fn new_session_starts_enabled() {
    assert!(Session::new(Settings::default()).is_enabled());
}

#[test]
fn toggle_returns_new_state() {
    let session = Session::new(Settings::default());
    assert!(!session.toggle());
    assert!(!session.is_enabled());
    assert!(session.toggle());
    assert!(session.is_enabled());
}

#[test]
fn settings_returns_stored_settings() {
    let session = Session::new(Settings::default());
    assert_eq!(session.settings(), Settings::default());
}

#[test]
fn update_applies_change_to_settings() {
    let session = Session::new(Settings::default());
    session.update(|settings| settings.sound_enabled = false);
    assert!(!session.settings().sound_enabled);
}

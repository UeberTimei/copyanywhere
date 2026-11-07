use super::*;

#[test]
fn menu_ids_map_to_commands() {
    assert_eq!(menu_command("toggle"), Some(TrayCommand::Toggle));
    assert_eq!(menu_command("settings"), Some(TrayCommand::Settings));
    assert_eq!(menu_command("exit"), Some(TrayCommand::Exit));
    assert_eq!(menu_command("unknown"), None);
}

#[test]
fn left_button_release_toggles() {
    assert_eq!(
        click_command(MouseButton::Left, MouseButtonState::Up),
        Some(TrayCommand::Toggle)
    );
}

#[test]
fn other_clicks_are_ignored() {
    assert_eq!(
        click_command(MouseButton::Right, MouseButtonState::Up),
        None
    );
    assert_eq!(
        click_command(MouseButton::Left, MouseButtonState::Down),
        None
    );
}

#[test]
fn toggle_label_names_the_next_action() {
    assert_eq!(toggle_label(true), "Disable");
    assert_eq!(toggle_label(false), "Enable");
}

use super::*;

#[test]
fn left_button_release_toggles() {
    assert_eq!(
        click_command(MouseButton::Left, MouseButtonState::Up),
        Some(TrayCommand::Toggle)
    );
}

#[test]
fn right_button_release_opens_the_menu() {
    assert_eq!(
        click_command(MouseButton::Right, MouseButtonState::Up),
        Some(TrayCommand::Menu)
    );
}

#[test]
fn other_clicks_are_ignored() {
    assert_eq!(
        click_command(MouseButton::Middle, MouseButtonState::Up),
        None
    );
    assert_eq!(
        click_command(MouseButton::Left, MouseButtonState::Down),
        None
    );
}

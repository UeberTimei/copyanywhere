use super::*;

#[test]
fn left_button_down_maps_to_down() {
    assert_eq!(
        to_event(WM_LBUTTONDOWN, 3, 4, 99),
        Some(MouseEvent::Down {
            x: 3,
            y: 4,
            at_ms: 99
        })
    );
}

#[test]
fn left_button_up_maps_to_up() {
    assert_eq!(
        to_event(WM_LBUTTONUP, -5, 6, 100),
        Some(MouseEvent::Up {
            x: -5,
            y: 6,
            at_ms: 100
        })
    );
}

#[test]
fn other_messages_are_ignored() {
    assert_eq!(to_event(0x0200, 0, 0, 0), None);
    assert_eq!(to_event(0x0204, 0, 0, 0), None);
}

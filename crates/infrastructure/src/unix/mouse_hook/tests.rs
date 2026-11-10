use domain::selection::MouseEvent;

use super::edge;

#[test]
fn pressing_the_button_maps_to_down() {
    assert_eq!(
        edge(false, true, (3, 4), 99),
        Some(MouseEvent::Down {
            x: 3,
            y: 4,
            at_ms: 99
        })
    );
}

#[test]
fn releasing_the_button_maps_to_up() {
    assert_eq!(
        edge(true, false, (-5, 6), 100),
        Some(MouseEvent::Up {
            x: -5,
            y: 6,
            at_ms: 100
        })
    );
}

#[test]
fn holding_or_staying_up_is_ignored() {
    assert_eq!(edge(true, true, (0, 0), 0), None);
    assert_eq!(edge(false, false, (0, 0), 0), None);
}

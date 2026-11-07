use super::geometry::{Area, pixel_size, popup_origin};

const SCREEN: Area = Area {
    left: 0,
    top: 0,
    right: 1920,
    bottom: 1080,
};
const SIZE: (i32, i32) = (96, 32);

#[test]
fn sits_above_and_right_of_cursor() {
    assert_eq!(popup_origin((500, 500), SIZE, SCREEN), (512, 456));
}

#[test]
fn drops_below_when_no_room_above() {
    assert_eq!(popup_origin((500, 10), SIZE, SCREEN), (512, 22));
}

#[test]
fn clamps_to_right_edge() {
    assert_eq!(popup_origin((1900, 500), SIZE, SCREEN).0, 1824);
}

#[test]
fn follows_a_monitor_on_the_left() {
    let left = Area {
        left: -1920,
        top: 0,
        right: 0,
        bottom: 1080,
    };
    assert_eq!(popup_origin((-100, 500), SIZE, left).0, -96);
}

#[test]
fn pixel_size_scales_with_dpi() {
    assert_eq!(pixel_size(96), (96, 32));
    assert_eq!(pixel_size(144), (144, 48));
}

#[test]
fn fade_is_quick_in_solid_in_the_middle_and_out_at_the_end() {
    use super::fade::{LIFETIME, opacity};
    use std::time::Duration;
    assert_eq!(opacity(Duration::ZERO), 0.0);
    assert_eq!(opacity(Duration::from_millis(500)), 1.0);
    assert_eq!(opacity(LIFETIME), 0.0);
    assert!(opacity(LIFETIME - Duration::from_millis(90)) < 1.0);
}

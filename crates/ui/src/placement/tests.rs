use super::{Area, above_right, corner_at, pixel_size};

const SCREEN: Area = Area {
    left: 0,
    top: 0,
    right: 1920,
    bottom: 1040,
};
const SIZE: (i32, i32) = (96, 32);

#[test]
fn toast_sits_above_and_right_of_cursor() {
    assert_eq!(above_right((500, 500), SIZE, SCREEN), (512, 456));
}

#[test]
fn toast_drops_below_when_no_room_above() {
    assert_eq!(above_right((500, 10), SIZE, SCREEN), (512, 22));
}

#[test]
fn toast_clamps_to_right_edge() {
    assert_eq!(above_right((1900, 500), SIZE, SCREEN).0, 1824);
}

#[test]
fn toast_follows_a_monitor_on_the_left() {
    let left = Area {
        left: -1920,
        top: 0,
        right: 0,
        bottom: 1080,
    };
    assert_eq!(above_right((-100, 500), SIZE, left).0, -96);
}

#[test]
fn window_has_its_corner_at_the_cursor() {
    assert_eq!(corner_at((1900, 1030), (440, 560), SCREEN), (1460, 470));
}

#[test]
fn window_stays_on_screen_near_the_top_left() {
    assert_eq!(corner_at((100, 100), (440, 560), SCREEN), (0, 0));
}

#[test]
fn pixel_size_scales_with_dpi() {
    assert_eq!(pixel_size((96.0, 32.0), 96), (96, 32));
    assert_eq!(pixel_size((96.0, 32.0), 144), (144, 48));
}

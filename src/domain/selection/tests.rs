use super::*;

fn down(x: i32, y: i32, at_ms: u64) -> MouseEvent {
    MouseEvent::Down { x, y, at_ms }
}

fn up(x: i32, y: i32, at_ms: u64) -> MouseEvent {
    MouseEvent::Up { x, y, at_ms }
}

fn click(tracker: &mut GestureTracker, x: i32, y: i32, at_ms: u64) -> Option<Gesture> {
    tracker.feed(down(x, y, at_ms));
    tracker.feed(up(x, y, at_ms + 10))
}

#[test]
fn drag_when_moved_exactly_four_px_euclidean() {
    let mut tracker = GestureTracker::default();
    tracker.feed(down(0, 0, 0));
    assert_eq!(tracker.feed(up(4, 0, 50)), Some(Gesture::Drag));
}

#[test]
fn drag_when_diagonal_three_three_euclidean() {
    let mut tracker = GestureTracker::default();
    tracker.feed(down(0, 0, 0));
    assert_eq!(tracker.feed(up(3, 3, 50)), Some(Gesture::Drag));
}

#[test]
fn no_drag_when_moved_three_px_on_one_axis() {
    let mut tracker = GestureTracker::default();
    tracker.feed(down(0, 0, 0));
    assert_eq!(tracker.feed(up(3, 0, 50)), None);
}

#[test]
fn single_click_is_none() {
    let mut tracker = GestureTracker::default();
    assert_eq!(click(&mut tracker, 0, 0, 0), None);
}

#[test]
fn second_click_within_500ms_and_4px_is_multi_click() {
    let mut tracker = GestureTracker::default();
    click(&mut tracker, 0, 0, 0);
    assert_eq!(click(&mut tracker, 1, 1, 200), Some(Gesture::MultiClick));
}

#[test]
fn second_click_exactly_500ms_after_first_is_multi_click() {
    let mut tracker = GestureTracker::default();
    click(&mut tracker, 0, 0, 0);
    assert_eq!(click(&mut tracker, 0, 0, 500), Some(Gesture::MultiClick));
}

#[test]
fn second_click_exactly_4px_away_is_multi_click() {
    let mut tracker = GestureTracker::default();
    click(&mut tracker, 0, 0, 0);
    assert_eq!(click(&mut tracker, 4, 0, 100), Some(Gesture::MultiClick));
}

#[test]
fn second_click_over_4px_away_is_none() {
    let mut tracker = GestureTracker::default();
    click(&mut tracker, 0, 0, 0);
    assert_eq!(click(&mut tracker, 5, 0, 100), None);
}

#[test]
fn slow_second_click_600ms_is_none() {
    let mut tracker = GestureTracker::default();
    click(&mut tracker, 0, 0, 0);
    assert_eq!(click(&mut tracker, 0, 0, 600), None);
}

#[test]
fn triple_click_third_is_multi_click() {
    let mut tracker = GestureTracker::default();
    click(&mut tracker, 0, 0, 0);
    click(&mut tracker, 0, 0, 200);
    assert_eq!(click(&mut tracker, 0, 0, 400), Some(Gesture::MultiClick));
}

#[test]
fn drag_after_click_is_drag() {
    let mut tracker = GestureTracker::default();
    click(&mut tracker, 0, 0, 0);
    tracker.feed(down(0, 0, 100));
    assert_eq!(tracker.feed(up(10, 0, 150)), Some(Gesture::Drag));
}

#[test]
fn drag_resets_click_chain() {
    let mut tracker = GestureTracker::default();
    click(&mut tracker, 0, 0, 0);
    tracker.feed(down(0, 0, 100));
    assert_eq!(tracker.feed(up(20, 0, 110)), Some(Gesture::Drag));
    assert_eq!(click(&mut tracker, 0, 0, 200), None);
}

#[test]
fn up_without_down_is_none() {
    let mut tracker = GestureTracker::default();
    assert_eq!(tracker.feed(up(0, 0, 0)), None);
}

#[test]
fn second_up_without_new_down_is_none() {
    let mut tracker = GestureTracker::default();
    tracker.feed(down(0, 0, 0));
    tracker.feed(up(0, 0, 10));
    assert_eq!(tracker.feed(up(0, 0, 20)), None);
}

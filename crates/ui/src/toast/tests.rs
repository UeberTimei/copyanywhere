use super::fade::{LIFETIME, opacity};
use std::time::Duration;

#[test]
fn fade_is_quick_in_solid_in_the_middle_and_out_at_the_end() {
    assert_eq!(opacity(Duration::ZERO), 0.0);
    assert_eq!(opacity(Duration::from_millis(500)), 1.0);
    assert_eq!(opacity(LIFETIME), 0.0);
    assert!(opacity(LIFETIME - Duration::from_millis(90)) < 1.0);
}

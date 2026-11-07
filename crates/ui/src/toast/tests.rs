use super::fade::{LIFETIME, opacity};
use std::time::Duration;

#[test]
fn fade_is_quick_in_solid_in_the_middle_and_out_at_the_end() {
    assert_eq!(opacity(Duration::ZERO), 0.0);
    assert_eq!(opacity(Duration::from_millis(500)), 1.0);
    assert_eq!(opacity(LIFETIME), 0.0);
    assert!(opacity(LIFETIME - Duration::from_millis(90)) < 1.0);
}

#[test]
fn tint_scales_only_the_alpha_channel() {
    use super::fade::tint;
    use gpui::rgba;
    assert_eq!(tint(0x232325ff, 1.0), rgba(0x232325ff));
    assert_eq!(tint(0x232325ff, 0.0), rgba(0x23232500));
    assert_eq!(tint(0xededed14, 0.5), rgba(0xededed0a));
}

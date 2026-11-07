use super::*;

fn pixel(buffer: &[u8], x: usize, y: usize) -> [u8; 4] {
    let start = (y * SIZE as usize + x) * 4;
    [
        buffer[start],
        buffer[start + 1],
        buffer[start + 2],
        buffer[start + 3],
    ]
}

#[test]
fn buffer_holds_four_bytes_per_pixel() {
    assert_eq!(rgba(true).len(), 32 * 32 * 4);
    assert_eq!(rgba(false).len(), 32 * 32 * 4);
}

#[test]
fn centre_is_accent_when_enabled() {
    assert_eq!(pixel(&rgba(true), 16, 16), [0x46, 0xc2, 0xa1, 0xff]);
}

#[test]
fn centre_is_muted_when_disabled() {
    assert_eq!(pixel(&rgba(false), 16, 16), [0x85, 0x85, 0x8b, 0xff]);
}

#[test]
fn corner_is_transparent() {
    assert_eq!(pixel(&rgba(true), 0, 0)[3], 0);
    assert_eq!(pixel(&rgba(false), 31, 31)[3], 0);
}

#[test]
fn icon_builds_for_both_states() {
    assert!(icon(true).is_ok());
    assert!(icon(false).is_ok());
}

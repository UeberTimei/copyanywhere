use super::{COPIED_FLAGS, play_copied};
use windows::Win32::Media::Audio::SND_ASYNC;

#[test]
fn copied_flags_include_async() {
    assert_ne!(COPIED_FLAGS.0 & SND_ASYNC.0, 0);
}

#[test]
fn play_copied_does_not_panic() {
    play_copied();
}

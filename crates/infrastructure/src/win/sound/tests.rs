use super::{COPIED_FLAGS, play_copied};
use windows::Win32::Media::Audio::{SND_ASYNC, SND_MEMORY};

#[test]
fn copied_flags_play_from_memory_without_blocking() {
    assert_ne!(COPIED_FLAGS.0 & SND_ASYNC.0, 0);
    assert_ne!(COPIED_FLAGS.0 & SND_MEMORY.0, 0);
}

#[test]
fn play_copied_does_not_panic() {
    play_copied();
}

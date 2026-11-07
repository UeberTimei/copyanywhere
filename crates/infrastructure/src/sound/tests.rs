use super::{COPIED_FLAGS, SAMPLE_RATE, TONE_MS, play_copied, tone_wav};
use windows::Win32::Media::Audio::{SND_ASYNC, SND_MEMORY};

#[test]
fn copied_flags_play_from_memory_without_blocking() {
    assert_ne!(COPIED_FLAGS.0 & SND_ASYNC.0, 0);
    assert_ne!(COPIED_FLAGS.0 & SND_MEMORY.0, 0);
}

#[test]
fn wav_has_riff_header_and_expected_length() {
    let wav = tone_wav();
    let samples = (SAMPLE_RATE * TONE_MS / 1000) as usize;
    assert_eq!(&wav[..4], b"RIFF");
    assert_eq!(&wav[8..12], b"WAVE");
    assert_eq!(wav.len(), 44 + samples * 2);
}

#[test]
fn play_copied_does_not_panic() {
    play_copied();
}

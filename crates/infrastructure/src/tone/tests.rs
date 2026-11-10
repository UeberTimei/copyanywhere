use super::{SAMPLE_RATE, TOTAL_MS, tone_wav};

#[test]
fn wav_has_riff_header_and_expected_length() {
    let wav = tone_wav();
    let samples = (SAMPLE_RATE * TOTAL_MS / 1000) as usize;
    assert_eq!(&wav[..4], b"RIFF");
    assert_eq!(&wav[8..12], b"WAVE");
    assert_eq!(wav.len(), 44 + samples * 2);
}

#[test]
fn both_notes_are_audible() {
    let wav = tone_wav();
    let peak = |from: usize, to: usize| {
        wav[44..]
            .as_chunks::<2>()
            .0
            .iter()
            .skip(from)
            .take(to - from)
            .map(|pair| i16::from_le_bytes([pair[0], pair[1]]).unsigned_abs())
            .max()
            .unwrap_or(0)
    };
    let second_note_start = (SAMPLE_RATE as usize * 150) / 1000;
    assert!(peak(0, 2000) > 1000);
    assert!(peak(second_note_start, second_note_start + 2000) > 1000);
}

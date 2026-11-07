use std::sync::OnceLock;

use windows::Win32::Media::Audio::{PlaySoundW, SND_ASYNC, SND_FLAGS, SND_MEMORY, SND_NODEFAULT};
use windows::core::PCWSTR;

const COPIED_FLAGS: SND_FLAGS = SND_FLAGS(SND_MEMORY.0 | SND_ASYNC.0 | SND_NODEFAULT.0);
const SAMPLE_RATE: u32 = 44_100;
const VOLUME: f32 = 0.22;
// D6 then E6, the second note enters while the first is still ringing
const NOTES: [Note; 2] = [
    Note {
        hz: 1174.66,
        start_ms: 0,
        length_ms: 140,
    },
    Note {
        hz: 1318.51,
        start_ms: 85,
        length_ms: 190,
    },
];
const TOTAL_MS: u32 = 275;
const ATTACK_MS: f32 = 6.0;
const DECAY_PER_SECOND: f32 = 11.0;

struct Note {
    hz: f32,
    start_ms: u32,
    length_ms: u32,
}

static TONE: OnceLock<Vec<u8>> = OnceLock::new();

pub fn play_copied() {
    let tone = TONE.get_or_init(tone_wav);
    unsafe {
        let _ = PlaySoundW(PCWSTR(tone.as_ptr().cast()), None, COPIED_FLAGS);
    }
}

fn tone_wav() -> Vec<u8> {
    let samples = SAMPLE_RATE * TOTAL_MS / 1000;
    let data_len = samples * 2;
    let mut wav = Vec::with_capacity(44 + data_len as usize);
    wav.extend(b"RIFF");
    wav.extend((36 + data_len).to_le_bytes());
    wav.extend(b"WAVEfmt ");
    wav.extend(16u32.to_le_bytes());
    wav.extend(1u16.to_le_bytes());
    wav.extend(1u16.to_le_bytes());
    wav.extend(SAMPLE_RATE.to_le_bytes());
    wav.extend((SAMPLE_RATE * 2).to_le_bytes());
    wav.extend(2u16.to_le_bytes());
    wav.extend(16u16.to_le_bytes());
    wav.extend(b"data");
    wav.extend(data_len.to_le_bytes());
    wav.extend((0..samples).flat_map(|index| mixed(index).to_le_bytes()));
    wav
}

fn mixed(index: u32) -> i16 {
    let seconds = index as f32 / SAMPLE_RATE as f32;
    let level: f32 = NOTES.iter().map(|note| voice(note, seconds)).sum();
    (level.clamp(-1.0, 1.0) * VOLUME * f32::from(i16::MAX)) as i16
}

fn voice(note: &Note, seconds: f32) -> f32 {
    let local = seconds - note.start_ms as f32 / 1000.0;
    if !(0.0..note.length_ms as f32 / 1000.0).contains(&local) {
        return 0.0;
    }
    let attack = (local * 1000.0 / ATTACK_MS).min(1.0);
    let ring = (-DECAY_PER_SECOND * local).exp();
    let phase = std::f32::consts::TAU * note.hz * local;
    (phase.sin() + 0.25 * (2.0 * phase).sin()) * attack * ring * 0.8
}

#[cfg(test)]
mod tests;

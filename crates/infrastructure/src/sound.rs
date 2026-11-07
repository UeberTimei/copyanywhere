use std::sync::OnceLock;

use windows::Win32::Media::Audio::{PlaySoundW, SND_ASYNC, SND_FLAGS, SND_MEMORY, SND_NODEFAULT};
use windows::core::PCWSTR;

const COPIED_FLAGS: SND_FLAGS = SND_FLAGS(SND_MEMORY.0 | SND_ASYNC.0 | SND_NODEFAULT.0);
const SAMPLE_RATE: u32 = 22_050;
const TONE_HZ: f32 = 880.0;
const TONE_MS: u32 = 90;
const VOLUME: f32 = 0.25;

static TONE: OnceLock<Vec<u8>> = OnceLock::new();

pub fn play_copied() {
    let tone = TONE.get_or_init(tone_wav);
    unsafe {
        let _ = PlaySoundW(PCWSTR(tone.as_ptr().cast()), None, COPIED_FLAGS);
    }
}

fn tone_wav() -> Vec<u8> {
    let samples = SAMPLE_RATE * TONE_MS / 1000;
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
    wav.extend((0..samples).flat_map(|index| sample(index, samples).to_le_bytes()));
    wav
}

fn sample(index: u32, total: u32) -> i16 {
    let seconds = index as f32 / SAMPLE_RATE as f32;
    let fade = 1.0 - index as f32 / total as f32;
    let wave = (std::f32::consts::TAU * TONE_HZ * seconds).sin();
    (wave * fade * VOLUME * f32::from(i16::MAX)) as i16
}

#[cfg(test)]
mod tests;

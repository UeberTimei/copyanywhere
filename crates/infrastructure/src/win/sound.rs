use std::sync::OnceLock;

use windows::Win32::Media::Audio::{PlaySoundW, SND_ASYNC, SND_FLAGS, SND_MEMORY, SND_NODEFAULT};
use windows::core::PCWSTR;

use crate::tone::tone_wav;

#[cfg(test)]
mod tests;

const COPIED_FLAGS: SND_FLAGS = SND_FLAGS(SND_MEMORY.0 | SND_ASYNC.0 | SND_NODEFAULT.0);

static TONE: OnceLock<Vec<u8>> = OnceLock::new();

pub fn play_copied() {
    let tone = TONE.get_or_init(tone_wav);
    unsafe {
        let _ = PlaySoundW(PCWSTR(tone.as_ptr().cast()), None, COPIED_FLAGS);
    }
}

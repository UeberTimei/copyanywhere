#![allow(dead_code)]

use windows::Win32::Media::Audio::{PlaySoundW, SND_ALIAS, SND_ASYNC, SND_FLAGS, SND_NODEFAULT};
use windows::core::w;

const COPIED_FLAGS: SND_FLAGS = SND_FLAGS(SND_ALIAS.0 | SND_ASYNC.0 | SND_NODEFAULT.0);

pub fn play_copied() {
    unsafe {
        let _ = PlaySoundW(w!("SystemAsterisk"), None, COPIED_FLAGS);
    }
}

#[cfg(test)]
mod tests;

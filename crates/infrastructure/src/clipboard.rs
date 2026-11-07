#![allow(dead_code)]

use windows::Win32::System::DataExchange::{
    GetClipboardSequenceNumber, IsClipboardFormatAvailable,
};
use windows::Win32::System::Ole::CF_UNICODETEXT;

#[cfg(test)]
mod tests;

pub fn sequence() -> u32 {
    unsafe { GetClipboardSequenceNumber() }
}

pub fn has_text() -> bool {
    unsafe { IsClipboardFormatAvailable(CF_UNICODETEXT.0 as u32).is_ok() }
}

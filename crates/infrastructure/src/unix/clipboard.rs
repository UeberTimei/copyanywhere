use std::hash::{DefaultHasher, Hash, Hasher};

use arboard::Clipboard;

#[cfg(test)]
mod tests;

/// Stands in for Windows' clipboard sequence number: a digest of the text, 0 when there is none.
// shortcut: copying text that is already on the clipboard looks like no copy, use a change counter (NSPasteboard.changeCount on macOS) if that matters
pub fn sequence() -> u32 {
    text().map_or(0, |text| digest(&text))
}

pub fn has_text() -> bool {
    text().is_some_and(|text| !text.is_empty())
}

fn text() -> Option<String> {
    Clipboard::new().ok()?.get_text().ok()
}

fn digest(text: &str) -> u32 {
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    hasher.finish() as u32
}

use std::sync::mpsc::Sender;

use application::ports::{Clipboard, Feedback, Foreground};
use infrastructure::{clipboard, foreground, keyboard, mouse_hook, sound};

pub struct SystemClipboard;

impl Clipboard for SystemClipboard {
    fn sequence(&self) -> u32 {
        clipboard::sequence()
    }

    fn has_text(&self) -> bool {
        clipboard::has_text()
    }

    fn send_copy(&self) {
        keyboard::send_copy_shortcut();
    }
}

pub struct SystemForeground;

impl Foreground for SystemForeground {
    fn program(&self) -> Option<String> {
        foreground::foreground_program()
    }
}

pub struct ToastFeedback {
    pub toasts: Sender<(i32, i32)>,
}

impl Feedback for ToastFeedback {
    fn copied(&self, toast: bool, sound: bool) {
        if sound {
            sound::play_copied();
        }
        if toast {
            self.toasts.send(mouse_hook::cursor_position()).ok();
        }
    }
}

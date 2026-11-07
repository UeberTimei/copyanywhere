use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use domain::policy::should_copy;

use crate::ports::{Clipboard, Feedback, Foreground};
use crate::session::Session;

const WAIT: Duration = Duration::from_millis(300);
const SETTLE: Duration = Duration::from_millis(80);
const POLL: Duration = Duration::from_millis(10);

pub struct AutoCopy<C, F, B> {
    clipboard: C,
    foreground: F,
    feedback: B,
    session: Arc<Session>,
}

impl<C: Clipboard, F: Foreground, B: Feedback> AutoCopy<C, F, B> {
    pub fn new(clipboard: C, foreground: F, feedback: B, session: Arc<Session>) -> Self {
        Self {
            clipboard,
            foreground,
            feedback,
            session,
        }
    }

    pub fn on_gesture(&self) {
        let settings = self.session.settings();
        let program = self.foreground.program();
        if !should_copy(
            self.session.is_enabled(),
            program.as_deref(),
            &settings.excluded,
        ) {
            return;
        }
        thread::sleep(SETTLE);
        let before = self.clipboard.sequence();
        self.clipboard.send_copy();
        if self.wait_for_text(before) {
            self.feedback
                .copied(settings.toast_enabled, settings.sound_enabled);
        }
    }

    fn wait_for_text(&self, before: u32) -> bool {
        let deadline = Instant::now() + WAIT;
        while Instant::now() < deadline {
            if self.clipboard.sequence() != before && self.clipboard.has_text() {
                return true;
            }
            thread::sleep(POLL);
        }
        false
    }
}

#[cfg(test)]
mod tests;

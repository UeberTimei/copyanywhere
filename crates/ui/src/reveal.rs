use std::time::Duration;

use gpui_kit::{Animation, ease_out_quint};

use crate::theme;

/// Open/close state of the program picker, shared by every element that animates with it.
#[derive(Clone, Copy)]
pub struct Reveal {
    pub open: bool,
    /// Bumped on each toggle so every toggle starts a fresh animation; zero means "never toggled".
    pub toggles: usize,
}

impl Reveal {
    pub fn id(self, name: &'static str) -> (&'static str, usize) {
        (name, self.toggles)
    }

    pub fn animation(self) -> Animation {
        Animation::new(Duration::from_millis(theme::PICKER_ANIMATION_MS))
            .with_easing(ease_out_quint())
    }

    /// 0 = fully closed, 1 = fully open, given the raw animation time `t`.
    pub fn progress(self, t: f32) -> f32 {
        match (self.toggles, self.open) {
            (0, _) => 0.,
            (_, true) => t,
            (_, false) => 1. - t,
        }
    }
}

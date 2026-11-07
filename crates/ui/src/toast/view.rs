use std::time::Instant;

use gpui::{Context, IntoElement, Render, Window, div, prelude::*, px};

use super::fade::{self, tint};

const RAISED: u32 = 0x232325ff;
const HAIRLINE: u32 = 0xededed14;
const TEXT: u32 = 0xedededff;
const ACCENT: u32 = 0x46c2a1ff;

pub struct ToastView {
    born: Instant,
}

impl ToastView {
    pub fn new() -> Self {
        Self {
            born: Instant::now(),
        }
    }
}

impl Render for ToastView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let elapsed = self.born.elapsed();
        let opacity = fade::opacity(elapsed);
        div()
            .size_full()
            .flex()
            .items_center()
            .gap(px(8.))
            .px(px(12.))
            .rounded(px(8.))
            .bg(tint(RAISED, opacity))
            .border_1()
            .border_color(tint(HAIRLINE, opacity))
            .text_color(tint(TEXT, opacity))
            .text_size(px(13.))
            .font_family("Segoe UI")
            .child(div().size(px(8.)).rounded_full().bg(tint(ACCENT, opacity)))
            .child("Copied")
    }
}

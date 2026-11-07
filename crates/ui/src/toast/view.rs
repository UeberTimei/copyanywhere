use std::time::Instant;

use gpui::{Context, IntoElement, Render, Window, div, prelude::*, px, rgb, rgba};

use super::fade;

const RAISED: u32 = 0x232325;
const HAIRLINE: u32 = 0xededed14;
const TEXT: u32 = 0xededed;
const ACCENT: u32 = 0x46c2a1;

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
    fn render(&mut self, window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let elapsed = self.born.elapsed();
        if elapsed < fade::LIFETIME {
            window.request_animation_frame();
        }
        div()
            .size_full()
            .opacity(fade::opacity(elapsed))
            .flex()
            .items_center()
            .gap(px(8.))
            .px(px(12.))
            .rounded(px(8.))
            .bg(rgb(RAISED))
            .border_1()
            .border_color(rgba(HAIRLINE))
            .text_color(rgb(TEXT))
            .text_size(px(13.))
            .font_family("Segoe UI")
            .child(div().size(px(8.)).rounded_full().bg(rgb(ACCENT)))
            .child("Copied")
    }
}

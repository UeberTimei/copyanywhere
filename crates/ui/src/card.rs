use gpui_kit::{Div, FontWeight, IntoElement, div, prelude::*, px, rgb, rgba};

use crate::theme;

pub fn card() -> Div {
    div()
        .flex()
        .flex_col()
        .p(px(theme::CARD_PADDING))
        .rounded(px(theme::RADIUS_CARD))
        .bg(rgb(theme::CARD_BG))
        .border_1()
        .border_color(rgba(theme::HAIRLINE))
}

pub fn section(title: &'static str, body: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(theme::LABEL_GAP))
        .child(
            div()
                .text_size(px(12.))
                .line_height(px(16.))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgb(theme::TEXT_SECONDARY))
                .child(title),
        )
        .child(body)
}

pub fn muted_text(text: &'static str) -> Div {
    div()
        .text_size(px(12.))
        .line_height(px(16.))
        .text_color(rgb(theme::TEXT_MUTED))
        .child(text)
}

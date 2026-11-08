use gpui_kit::{
    ClickEvent, FontWeight, IntoElement, Window, WindowControlArea, div, prelude::*, px, rgb, rgba,
};

use crate::theme;

pub fn header(enabled: bool) -> impl IntoElement {
    let (status, dot) = if enabled {
        ("Auto-copy is on", theme::ACCENT)
    } else {
        ("Auto-copy is off", theme::TEXT_MUTED)
    };
    div()
        .flex()
        .items_center()
        .justify_between()
        .child(
            div()
                .flex_1()
                .flex()
                .items_center()
                .gap(px(8.))
                .window_control_area(WindowControlArea::Drag)
                .child(div().size(px(theme::DOT_SIZE)).rounded_full().bg(rgb(dot)))
                .child(
                    div()
                        .text_size(px(20.))
                        .line_height(px(24.))
                        .font_weight(FontWeight::MEDIUM)
                        .child(status),
                ),
        )
        .child(close_button())
}

fn close_button() -> impl IntoElement {
    div()
        .id("close")
        .size(px(28.))
        .flex()
        .items_center()
        .justify_center()
        .rounded(px(theme::RADIUS_CONTROL))
        .text_size(px(18.))
        .text_color(rgb(theme::TEXT_SECONDARY))
        .cursor_pointer()
        .hover(|style| style.bg(rgba(theme::HAIRLINE)).text_color(rgb(theme::TEXT)))
        .child("×")
        .on_click(|_: &ClickEvent, window: &mut Window, _| window.remove_window())
}

use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{ActiveTheme, IconName};
use gpui_kit::{App, FontWeight, Hsla, IntoElement, WindowControlArea, div, prelude::*, px, rgb};

use crate::theme;

pub fn header(enabled: bool, cx: &App) -> impl IntoElement {
    let (status, dot) = if enabled {
        ("Auto-copy is on", Hsla::from(rgb(theme::ACCENT)))
    } else {
        ("Auto-copy is off", cx.theme().muted_foreground)
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
                .gap_2()
                .window_control_area(WindowControlArea::Drag)
                .child(div().size(px(theme::DOT_SIZE)).rounded_full().bg(dot))
                .child(
                    div()
                        .text_xl()
                        .font_weight(FontWeight::MEDIUM)
                        .child(status),
                ),
        )
        .child(close_button())
}

fn close_button() -> impl IntoElement {
    Button::new("close")
        .ghost()
        .compact()
        .icon(IconName::Close)
        .on_click(|_, window, _| window.remove_window())
}

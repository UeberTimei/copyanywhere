use gpui_kit::component::ActiveTheme;
use gpui_kit::{App, Div, div, prelude::*};

pub fn muted(text: &'static str, cx: &App) -> Div {
    div()
        .text_xs()
        .text_color(cx.theme().muted_foreground)
        .child(text)
}

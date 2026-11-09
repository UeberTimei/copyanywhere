use gpui_kit::base::Disableable;
use gpui_kit::component::ActiveTheme;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::{Context, FontWeight, Hsla, IntoElement, Render, Window, div, prelude::*, px, rgb};

use crate::text::muted;
use crate::theme;
use crate::update_logic::plain_notes;
use crate::update_window::{Progress, UpdateView};

impl Render for UpdateView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let installing = matches!(self.progress, Progress::Installing);
        div()
            .size_full()
            .flex()
            .flex_col()
            .gap_4()
            .p_6()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .border_1()
            .border_color(cx.theme().border)
            .child(title(&self.release.version))
            .child(muted("What's new", cx))
            .child(
                div()
                    .id("notes")
                    .flex_1()
                    .overflow_y_scroll()
                    .text_sm()
                    .child(plain_notes(&self.release.notes)),
            )
            .children(match &self.progress {
                Progress::Failed(error) => Some(
                    div()
                        .text_xs()
                        .text_color(cx.theme().danger)
                        .child(format!("Update failed: {error}")),
                ),
                _ => None,
            })
            .child(buttons(installing, cx))
    }
}

fn title(version: &str) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .gap_2()
        .child(
            div()
                .size(px(theme::DOT_SIZE))
                .rounded_full()
                .bg(Hsla::from(rgb(theme::ACCENT))),
        )
        .child(
            div()
                .text_xl()
                .font_weight(FontWeight::MEDIUM)
                .child(format!("CopyAnywhere {version} is available")),
        )
}

fn buttons(installing: bool, cx: &Context<UpdateView>) -> impl IntoElement {
    div()
        .flex()
        .gap_2()
        .child(
            Button::new("install")
                .primary()
                .disabled(installing)
                .label(if installing {
                    "Installing…"
                } else {
                    "Install"
                })
                .on_click(cx.listener(|this, _, _, cx| this.install(cx))),
        )
        .child(
            Button::new("later")
                .outline()
                .disabled(installing)
                .label("Remind me later")
                .on_click(|_, window, _| window.remove_window()),
        )
        .child(
            Button::new("skip")
                .ghost()
                .disabled(installing)
                .label("Skip this version")
                .on_click(cx.listener(|this, _, window, _| {
                    this.skip();
                    window.remove_window();
                })),
        )
}

use std::rc::Rc;

use gpui::{
    App, ClickEvent, Context, FontWeight, IntoElement, Render, Window, div, prelude::*, px, rgb,
    rgba,
};

use crate::card::{card, section};
use crate::exclusion_list::{Callback, exclusion_card};
use crate::program_picker::program_picker;
use crate::settings_logic::{self, Feature};
use crate::settings_window::SettingsView;
use crate::theme;
use crate::toggle::Toggle;

impl Render for SettingsView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view = cx.entity();
        let remove: Callback = {
            let view = view.clone();
            Rc::new(move |name: &str, cx: &mut App| {
                view.update(cx, |this, cx| this.remove_program(name, cx))
            })
        };
        let pick: Callback = Rc::new(move |name: &str, cx: &mut App| {
            view.update(cx, |this, cx| this.add_program(name, cx))
        });
        let programs: Vec<String> = self.settings.excluded.iter().map(String::from).collect();
        let picker = self
            .picker_open
            .then(|| program_picker(&self.pickable(), &pick));

        div()
            .id("settings")
            .size_full()
            .flex()
            .flex_col()
            .gap(px(theme::CARD_GAP))
            .p(px(theme::WINDOW_PADDING))
            .bg(rgb(theme::WINDOW_BG))
            .text_color(rgb(theme::TEXT))
            .font_family(theme::FONT_FAMILY)
            .overflow_y_scroll()
            .child(header(self.enabled))
            .child(section(
                "General",
                card().child(toggle_row(
                    "Launch with Windows",
                    settings_logic::is_on(&self.settings, Feature::LaunchAtLogin),
                    Feature::LaunchAtLogin,
                    cx,
                )),
            ))
            .child(section(
                "Feedback",
                card()
                    .child(toggle_row(
                        "Play sound",
                        settings_logic::is_on(&self.settings, Feature::Sound),
                        Feature::Sound,
                        cx,
                    ))
                    .child(toggle_row(
                        "Show notification",
                        settings_logic::is_on(&self.settings, Feature::Notification),
                        Feature::Notification,
                        cx,
                    )),
            ))
            .child(section(
                "Excluded programs",
                div()
                    .flex()
                    .flex_col()
                    .gap(px(theme::LABEL_GAP))
                    .child(crate::card::muted_text(
                        "Auto-copy is skipped in these programs",
                    ))
                    .child(exclusion_card(&programs, &remove))
                    .child(add_button(cx))
                    .children(picker),
            ))
    }
}

fn header(enabled: bool) -> impl IntoElement {
    let (status, dot) = if enabled {
        ("Auto-copy is on", theme::ACCENT)
    } else {
        ("Auto-copy is off", theme::TEXT_MUTED)
    };
    div()
        .flex()
        .items_center()
        .gap(px(8.))
        .child(div().size(px(theme::DOT_SIZE)).rounded_full().bg(rgb(dot)))
        .child(
            div()
                .text_size(px(20.))
                .line_height(px(24.))
                .font_weight(FontWeight::MEDIUM)
                .child(status),
        )
}

fn toggle_row(
    label: &'static str,
    on: bool,
    feature: Feature,
    cx: &Context<SettingsView>,
) -> impl IntoElement {
    let toggle = Toggle::new(
        label,
        on,
        cx.listener(move |this, _: &ClickEvent, _, cx| this.toggle_feature(feature, cx)),
    );
    div()
        .h(px(theme::ROW_HEIGHT))
        .px(px(12.))
        .flex()
        .items_center()
        .justify_between()
        .rounded(px(theme::RADIUS_CONTROL))
        .text_size(px(14.))
        .line_height(px(20.))
        .child(label)
        .child(toggle)
}

fn add_button(cx: &Context<SettingsView>) -> impl IntoElement {
    div()
        .id("add-program")
        .h(px(32.))
        .px(px(12.))
        .flex()
        .items_center()
        .rounded(px(theme::RADIUS_CONTROL))
        .bg(rgb(theme::RAISED_BG))
        .border_1()
        .border_color(rgba(theme::HAIRLINE))
        .text_size(px(14.))
        .cursor_pointer()
        .child("Add program")
        .on_click(cx.listener(|this, _: &ClickEvent, _, cx| this.toggle_picker(cx)))
}

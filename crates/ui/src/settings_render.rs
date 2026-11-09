use std::f32::consts::FRAC_PI_2;
use std::rc::Rc;

use gpui_kit::component::ActiveTheme;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::group_box::{GroupBox, GroupBoxVariants};
use gpui_kit::component::switch::Switch;
use gpui_kit::component::{Icon, IconName};
use gpui_kit::{
    AnimationExt, App, Context, Hsla, IntoElement, Render, Window, div, prelude::*, radians, rgb,
};

use crate::exclusion_list::{Callback, exclusion_list};
use crate::program_picker::program_picker;
use crate::reveal::Reveal;
use crate::settings_header::header;
use crate::settings_logic::{self, Feature};
use crate::settings_window::SettingsView;
use crate::text::muted;
use crate::theme::{self, ACCENT_TINT};

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
        let programs = self.excluded_programs();
        let reveal = Reveal {
            open: self.picker_open,
            toggles: self.picker_toggles,
        };
        let picker =
            (reveal.toggles > 0).then(|| program_picker(&self.pickable(), &pick, reveal, cx));

        div()
            .id("settings")
            .size_full()
            .flex()
            .flex_col()
            .gap_4()
            .p_6()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .border_1()
            .border_color(cx.theme().border)
            .overflow_y_scroll()
            .child(header(self.enabled, cx))
            .child(
                GroupBox::new()
                    .fill()
                    .title("General")
                    .child(toggle_row(
                        "Launch with Windows",
                        Feature::LaunchAtLogin,
                        self,
                        cx,
                    ))
                    .child(toggle_row(
                        "Check for updates on launch",
                        Feature::CheckUpdates,
                        self,
                        cx,
                    )),
            )
            .child(
                GroupBox::new()
                    .fill()
                    .title("Feedback")
                    .child(toggle_row("Play sound", Feature::Sound, self, cx))
                    .child(toggle_row(
                        "Show notification",
                        Feature::Notification,
                        self,
                        cx,
                    )),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(muted("Auto-copy is skipped in these programs", cx))
                    .child(add_buttons(reveal, cx))
                    .children(picker)
                    .child(div().mt_2().child(exclusion_list(&programs, &remove, cx))),
            )
    }
}

fn toggle_row(
    label: &'static str,
    feature: Feature,
    view: &SettingsView,
    cx: &Context<SettingsView>,
) -> impl IntoElement {
    Switch::new(label)
        .label(label)
        .checked(settings_logic::is_on(&view.settings, feature))
        .color(rgb(theme::ACCENT))
        .on_click(cx.listener(move |this, _: &bool, _, cx| this.toggle_feature(feature, cx)))
}

fn add_buttons(reveal: Reveal, cx: &Context<SettingsView>) -> impl IntoElement {
    div()
        .w_full()
        .mt_2()
        .flex()
        .gap_2()
        .child(add_current_button(reveal, cx))
        .child(
            Button::new("browse-program")
                .outline()
                .flex_1()
                .label("Browse a program")
                .on_click(cx.listener(|this, _, _, cx| this.browse_program(cx))),
        )
}

/// A ghost button inside a frame whose border and tint fade to the accent as the picker opens.
fn add_current_button(reveal: Reveal, cx: &Context<SettingsView>) -> impl IntoElement {
    let accent = Hsla::from(rgb(theme::ACCENT));
    let border = cx.theme().border;
    let arrow = Icon::new(IconName::ChevronRight).size_4().with_animation(
        reveal.id("add-program-arrow"),
        reveal.animation(),
        move |icon, t| icon.rotate(radians(FRAC_PI_2 * reveal.progress(t))),
    );
    div()
        .flex_1()
        .rounded(cx.theme().radius)
        .border_1()
        .child(
            Button::new("add-program")
                .ghost()
                .w_full()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child("Add current program")
                        .child(arrow),
                )
                .on_click(cx.listener(|this, _, _, cx| this.toggle_picker(cx))),
        )
        .with_animation(
            reveal.id("add-program-frame"),
            reveal.animation(),
            move |frame, t| {
                let progress = reveal.progress(t);
                frame
                    .border_color(border.blend(accent.opacity(progress)))
                    .bg(accent.opacity(ACCENT_TINT * progress))
            },
        )
}

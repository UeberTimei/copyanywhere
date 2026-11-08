use std::rc::Rc;

use gpui_kit::component::ActiveTheme;
use gpui_kit::component::button::Button;
use gpui_kit::component::group_box::{GroupBox, GroupBoxVariants};
use gpui_kit::component::switch::Switch;
use gpui_kit::{App, Context, IntoElement, Render, Window, div, prelude::*, rgb};

use crate::exclusion_list::{Callback, exclusion_list};
use crate::program_picker::program_picker;
use crate::settings_header::header;
use crate::settings_logic::{self, Feature};
use crate::settings_window::SettingsView;
use crate::text::muted;
use crate::theme;

impl Render for SettingsView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view = cx.entity();
        let remove: Callback = {
            let view = view.clone();
            Rc::new(move |name: &str, cx: &mut App| {
                view.update(cx, |this, cx| this.remove_program(name, cx))
            })
        };
        let browse_view = view.clone();
        let pick: Callback = Rc::new(move |name: &str, cx: &mut App| {
            view.update(cx, |this, cx| this.add_program(name, cx))
        });
        let browse = move |cx: &mut App| browse_view.update(cx, |this, cx| this.browse_program(cx));
        let programs: Vec<String> = self.settings.excluded.iter().map(String::from).collect();
        let picker = self
            .picker_open
            .then(|| program_picker(&self.pickable(), &pick, browse, cx));

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
            .child(GroupBox::new().fill().title("General").child(toggle_row(
                "Launch with Windows",
                Feature::LaunchAtLogin,
                self,
                cx,
            )))
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
                    .gap_2()
                    .child(muted("Auto-copy is skipped in these programs", cx))
                    .child(exclusion_list(&programs, &remove, cx))
                    .child(add_button(cx))
                    .children(picker),
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

fn add_button(cx: &Context<SettingsView>) -> impl IntoElement {
    Button::new("add-program")
        .outline()
        .label("Add program")
        .on_click(cx.listener(|this, _, _, cx| this.toggle_picker(cx)))
}

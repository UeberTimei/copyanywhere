use std::rc::Rc;

use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::group_box::{GroupBox, GroupBoxVariants};
use gpui_kit::{App, Div, Stateful, div, prelude::*, px};

use crate::exclusion_list::Callback;
use crate::text::muted;
use crate::theme;

pub fn program_picker(
    programs: &[String],
    on_pick: &Callback,
    on_browse: impl Fn(&mut App) + 'static,
    cx: &App,
) -> Stateful<Div> {
    let group = if programs.is_empty() {
        GroupBox::new()
            .outline()
            .child(muted("No running programs to add", cx))
    } else {
        programs
            .iter()
            .enumerate()
            .fold(GroupBox::new().outline(), |group, (index, name)| {
                group.child(pick_button(index, name, on_pick))
            })
    };
    div()
        .id("program-picker")
        .max_h(px(theme::PICKER_MAX_HEIGHT))
        .overflow_y_scroll()
        .child(group)
        .child(
            Button::new("browse-program")
                .outline()
                .label("Browse for program…")
                .on_click(move |_, _, cx| on_browse(cx)),
        )
}

fn pick_button(index: usize, name: &str, on_pick: &Callback) -> Button {
    let label = name.to_owned();
    let on_pick = Rc::clone(on_pick);
    Button::new(("pick", index))
        .ghost()
        .label(label.clone())
        .on_click(move |_, _, cx| on_pick(&label, cx))
}

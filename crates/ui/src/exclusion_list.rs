use std::rc::Rc;

use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::group_box::{GroupBox, GroupBoxVariants};
use gpui_kit::{App, Div, div, prelude::*, px};

use crate::text::muted;

pub type Callback = Rc<dyn Fn(&str, &mut App)>;

pub fn exclusion_list(programs: &[String], on_remove: &Callback, cx: &App) -> GroupBox {
    let group = GroupBox::new().fill();
    if programs.is_empty() {
        return group.child(muted("No excluded programs", cx));
    }
    programs
        .iter()
        .enumerate()
        .fold(group, |group, (index, name)| {
            group.child(program_row(name).child(remove_button(index, name, on_remove)))
        })
}

pub fn program_row(name: &str) -> Div {
    div()
        .h(px(36.))
        .flex()
        .items_center()
        .justify_between()
        .child(name.to_owned())
}

fn remove_button(index: usize, name: &str, on_remove: &Callback) -> Button {
    let name = name.to_owned();
    let on_remove = Rc::clone(on_remove);
    Button::new(("remove", index))
        .ghost()
        .compact()
        .label("Remove")
        .on_click(move |_, _, cx| on_remove(&name, cx))
}

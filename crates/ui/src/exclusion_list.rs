use std::rc::Rc;

use gpui_kit::{App, Div, div, prelude::*, px, rgb};

use crate::card::{card, muted_text};
use crate::theme;

pub type Callback = Rc<dyn Fn(&str, &mut App)>;

pub fn exclusion_card(programs: &[String], on_remove: &Callback) -> Div {
    if programs.is_empty() {
        return card().child(muted_text("No excluded programs"));
    }
    programs
        .iter()
        .enumerate()
        .fold(card(), |list, (index, name)| {
            list.child(program_row(name).child(remove_button(index, name, on_remove)))
        })
}

pub fn program_row(name: &str) -> Div {
    div()
        .h(px(theme::PROGRAM_ROW_HEIGHT))
        .px(px(12.))
        .flex()
        .items_center()
        .justify_between()
        .rounded(px(theme::RADIUS_CONTROL))
        .hover(|row| row.bg(rgb(theme::RAISED_BG)))
        .text_size(px(14.))
        .text_color(rgb(theme::TEXT))
        .child(name.to_owned())
}

fn remove_button(index: usize, name: &str, on_remove: &Callback) -> impl IntoElement {
    let name = name.to_owned();
    let on_remove = Rc::clone(on_remove);
    div()
        .id(("remove", index))
        .text_size(px(12.))
        .text_color(rgb(theme::TEXT_MUTED))
        .cursor_pointer()
        .child("Remove")
        .on_click(move |_, _, cx| on_remove(&name, cx))
}

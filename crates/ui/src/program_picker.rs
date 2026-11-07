use std::rc::Rc;

use gpui::{Div, IntoElement, Stateful, div, prelude::*, px, rgb, rgba};

use crate::card::muted_text;
use crate::exclusion_list::{Callback, program_row};
use crate::theme;

pub fn program_picker(programs: &[String], on_pick: &Callback) -> Stateful<Div> {
    let list = div()
        .id("program-picker")
        .flex()
        .flex_col()
        .p(px(theme::CARD_PADDING))
        .rounded(px(theme::RADIUS_CARD))
        .bg(rgb(theme::RAISED_BG))
        .border_1()
        .border_color(rgba(theme::HAIRLINE))
        .max_h(px(
            theme::PROGRAM_ROW_HEIGHT * theme::PROGRAM_LIST_ROWS + 2. * theme::CARD_PADDING
        ))
        .overflow_y_scroll();
    if programs.is_empty() {
        return list.child(muted_text("No running programs to add"));
    }
    programs
        .iter()
        .enumerate()
        .fold(list, |list, (index, name)| {
            list.child(pick_row(index, name, on_pick))
        })
}

fn pick_row(index: usize, name: &str, on_pick: &Callback) -> impl IntoElement {
    let row = program_row(name);
    let name = name.to_owned();
    let on_pick = Rc::clone(on_pick);
    div()
        .id(("pick", index))
        .cursor_pointer()
        .on_click(move |_, _, cx| on_pick(&name, cx))
        .child(row)
}

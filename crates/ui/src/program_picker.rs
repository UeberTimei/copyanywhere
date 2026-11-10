use std::rc::Rc;

use domain::program::Program;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::group_box::{GroupBox, GroupBoxVariants};
use gpui_kit::{AnimationExt, App, IntoElement, div, prelude::*, px};

use crate::exclusion_list::Callback;
use crate::reveal::Reveal;
use crate::text::muted;
use crate::theme;

pub fn program_picker(
    programs: &[Program],
    on_pick: &Callback,
    reveal: Reveal,
    cx: &App,
) -> impl IntoElement + use<> {
    let group = if programs.is_empty() {
        GroupBox::new()
            .outline()
            .child(muted("No running programs to add", cx))
    } else {
        programs
            .iter()
            .enumerate()
            .fold(GroupBox::new().outline(), |group, (index, program)| {
                group.child(pick_button(index, program, on_pick))
            })
    };
    div()
        .id("program-picker")
        .w_full()
        .overflow_y_scroll()
        // Runs after the built-in scroll, so the window behind never scrolls, even at the list's edge.
        .on_scroll(|_, _, cx| cx.stop_propagation())
        .child(group)
        .with_animation(
            reveal.id("program-picker-reveal"),
            reveal.animation(),
            move |picker, t| {
                let progress = reveal.progress(t);
                picker
                    .max_h(px(theme::PICKER_MAX_HEIGHT * progress))
                    .mt(px(theme::SECTION_GAP * progress))
                    .opacity(progress)
            },
        )
}

fn pick_button(index: usize, program: &Program, on_pick: &Callback) -> Button {
    let exe = program.exe.clone();
    let on_pick = Rc::clone(on_pick);
    Button::new(("pick", index))
        .ghost()
        .label(program.label())
        .on_click(move |_, _, cx| on_pick(&exe, cx))
}

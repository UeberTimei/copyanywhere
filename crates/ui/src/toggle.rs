use gpui::{
    App, ClickEvent, ElementId, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb, rgba,
};

use crate::theme;

type ClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

#[derive(IntoElement)]
pub struct Toggle {
    id: ElementId,
    on: bool,
    on_click: ClickHandler,
}

impl Toggle {
    pub fn new(
        id: impl Into<ElementId>,
        on: bool,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            on,
            on_click: Box::new(on_click),
        }
    }
}

impl RenderOnce for Toggle {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let (track, knob, knob_left) = if self.on {
            (
                theme::ACCENT,
                theme::ON_ACCENT,
                theme::TOGGLE_WIDTH - theme::KNOB_SIZE - 4.,
            )
        } else {
            (theme::RAISED_BG, theme::TEXT_SECONDARY, 2.)
        };
        div()
            .id(self.id)
            .w(px(theme::TOGGLE_WIDTH))
            .h(px(theme::TOGGLE_HEIGHT))
            .rounded_full()
            .bg(rgb(track))
            .border_1()
            .border_color(rgba(theme::HAIRLINE))
            .cursor_pointer()
            .on_click(self.on_click)
            .child(
                div()
                    .absolute()
                    .top(px(1.))
                    .left(px(knob_left))
                    .size(px(theme::KNOB_SIZE))
                    .rounded_full()
                    .bg(rgb(knob)),
            )
    }
}

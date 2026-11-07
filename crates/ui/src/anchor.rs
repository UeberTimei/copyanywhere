use gpui::{
    App, AppContext, Bounds, Context, IntoElement, Render, Window, WindowBounds, WindowKind,
    WindowOptions, div, point, px, size,
};

struct Anchor;

impl Render for Anchor {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

// GPUI quits when its last window closes, so one hidden window keeps the tray app alive
pub fn open(cx: &mut App) {
    let options = WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(Bounds::new(
            point(px(0.), px(0.)),
            size(px(1.), px(1.)),
        ))),
        kind: WindowKind::PopUp,
        focus: false,
        show: false,
        ..Default::default()
    };
    let _ = cx.open_window(options, |_, cx| cx.new(|_| Anchor));
}

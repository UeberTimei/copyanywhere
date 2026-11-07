mod dpi;
mod geometry;
#[cfg(test)]
mod tests;
mod view;

use std::time::Duration;

use gpui::{App, AppContext, Bounds, WindowBounds, WindowKind, WindowOptions, point, px, size};

use geometry::{TOAST_SIZE, popup_origin};
use view::ToastView;

const VISIBLE_FOR: Duration = Duration::from_millis(1200);

pub fn show(cx: &mut App, at: (i32, i32)) {
    let (x, y) = popup_origin(at, dpi::system_scale());
    let bounds = Bounds::new(
        point(px(x), px(y)),
        size(px(TOAST_SIZE.0), px(TOAST_SIZE.1)),
    );
    let options = WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        focus: false,
        show: true,
        kind: WindowKind::PopUp,
        is_movable: false,
        is_resizable: false,
        is_minimizable: false,
        ..Default::default()
    };
    let Ok(handle) = cx.open_window(options, |_, cx| cx.new(|_| ToastView)) else {
        return;
    };
    cx.spawn(async move |cx| {
        cx.background_executor().timer(VISIBLE_FOR).await;
        handle
            .update(cx, |_, window, _| window.remove_window())
            .ok();
    })
    .detach();
}

mod geometry;
mod place;
#[cfg(test)]
mod tests;
mod view;

use std::time::Duration;

use gpui::{App, AppContext, Bounds, WindowBounds, WindowKind, WindowOptions, point, px, size};

use geometry::TOAST_SIZE;
use view::ToastView;

const VISIBLE_FOR: Duration = Duration::from_millis(1200);

pub fn show(cx: &mut App, at: (i32, i32)) {
    // GPUI falls back to a huge window when bounds miss the primary monitor, so
    // it opens on the primary one hidden and place::show_near moves it for real
    let bounds = Bounds::new(
        point(px(0.), px(0.)),
        size(px(TOAST_SIZE.0), px(TOAST_SIZE.1)),
    );
    let options = WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        focus: false,
        show: false,
        kind: WindowKind::PopUp,
        is_movable: false,
        is_resizable: false,
        is_minimizable: false,
        ..Default::default()
    };
    let opened = cx.open_window(options, |window, cx| {
        place::show_near(window, at);
        cx.new(|_| ToastView)
    });
    let Ok(handle) = opened else {
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

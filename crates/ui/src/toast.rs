mod fade;
#[cfg(test)]
mod tests;
mod view;

use std::time::{Duration, Instant};

use gpui::{
    App, AppContext, Bounds, WindowBackgroundAppearance, WindowBounds, WindowKind, WindowOptions,
    point, px, size,
};

use crate::native_window::{Spot, handle_of, show_at};
use crate::placement::above_right;
use view::ToastView;

const FRAME: Duration = Duration::from_millis(16);
const TOAST_SIZE: (f32, f32) = (96.0, 32.0);

pub fn show(cx: &mut App, at: (i32, i32)) {
    // GPUI opens it at the top-left of the primary monitor, fully transparent until faded in,
    // and show_at then moves it next to the cursor
    let bounds = Bounds::new(
        point(px(0.), px(0.)),
        size(px(TOAST_SIZE.0), px(TOAST_SIZE.1)),
    );
    let options = WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        focus: false,
        show: true,
        window_background: WindowBackgroundAppearance::Transparent,
        kind: WindowKind::PopUp,
        is_movable: false,
        is_resizable: false,
        is_minimizable: false,
        ..Default::default()
    };
    let Ok(handle) = cx.open_window(options, |_, cx| cx.new(|_| ToastView::new())) else {
        return;
    };
    let spot = Spot {
        at,
        logical_size: TOAST_SIZE,
        origin: above_right,
        activate: false,
    };
    cx.spawn(async move |cx| {
        if let Ok(Some(hwnd)) = handle.update(cx, |_, window, _| handle_of(window)) {
            show_at(hwnd, &spot);
        }
        let started = Instant::now();
        while started.elapsed() < fade::LIFETIME {
            cx.background_executor().timer(FRAME).await;
            handle.update(cx, |_, window, _| window.refresh()).ok();
        }
        handle
            .update(cx, |_, window, _| window.remove_window())
            .ok();
    })
    .detach();
}

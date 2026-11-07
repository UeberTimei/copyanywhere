mod fade;
#[cfg(test)]
mod tests;
mod view;

use gpui::{
    App, AppContext, Bounds, WindowBackgroundAppearance, WindowBounds, WindowKind, WindowOptions,
    point, px, size,
};

use crate::native_window::{Spot, show_at};
use crate::placement::above_right;
use view::ToastView;

const TOAST_SIZE: (f32, f32) = (96.0, 32.0);

pub fn show(cx: &mut App, at: (i32, i32)) {
    // GPUI falls back to a huge window when bounds miss the primary monitor, so
    // it opens on the primary one hidden and show_at moves it for real
    let bounds = Bounds::new(
        point(px(0.), px(0.)),
        size(px(TOAST_SIZE.0), px(TOAST_SIZE.1)),
    );
    let options = WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        focus: false,
        show: false,
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
    handle
        .update(cx, |_, window, _| {
            let spot = Spot {
                at,
                logical_size: TOAST_SIZE,
                origin: above_right,
                activate: false,
            };
            show_at(window, &spot)
        })
        .ok();
    cx.spawn(async move |cx| {
        cx.background_executor().timer(fade::LIFETIME).await;
        handle
            .update(cx, |_, window, _| window.remove_window())
            .ok();
    })
    .detach();
}

use std::rc::Rc;
use std::sync::Arc;

use domain::update::Release;
use gpui_kit::{
    AnyWindowHandle, App, Bounds, Context, WindowBounds, WindowOptions, open_window, prelude::*,
    px, size,
};

use crate::native_window::{Spot, handle_of, opening_origin, show_at};
use crate::placement::centered;
use crate::theme;

pub struct UpdateActions {
    pub release: Release,
    pub install: Arc<dyn Fn() -> Result<(), String> + Send + Sync>,
    pub skip: Rc<dyn Fn(&str)>,
}

pub(crate) enum Progress {
    Idle,
    Installing,
    Failed(String),
}

pub struct UpdateView {
    pub(crate) release: Release,
    pub(crate) progress: Progress,
    install: Arc<dyn Fn() -> Result<(), String> + Send + Sync>,
    skip: Rc<dyn Fn(&str)>,
}

pub fn open(cx: &mut App, actions: UpdateActions, at: (i32, i32)) -> Option<AnyWindowHandle> {
    let logical_size = (theme::UPDATE_WIDTH, theme::UPDATE_HEIGHT);
    let spot = Spot {
        at,
        logical_size,
        origin: centered,
        activate: true,
    };
    let bounds = Bounds::new(
        opening_origin(cx, &spot),
        size(px(logical_size.0), px(logical_size.1)),
    );
    let options = WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        titlebar: None,
        is_resizable: false,
        show: true,
        ..Default::default()
    };
    let (handle, _) =
        open_window(options, cx, |_, cx| cx.new(|_| UpdateView::new(actions))).ok()?;
    cx.spawn(async move |cx| {
        if let Ok(Some(hwnd)) = handle.update(cx, |_, window, _| handle_of(window)) {
            show_at(hwnd, &spot);
        }
    })
    .detach();
    Some(handle)
}

impl UpdateView {
    fn new(actions: UpdateActions) -> Self {
        Self {
            release: actions.release,
            progress: Progress::Idle,
            install: actions.install,
            skip: actions.skip,
        }
    }

    pub(crate) fn install(&mut self, cx: &mut Context<Self>) {
        self.progress = Progress::Installing;
        cx.notify();
        let install = Arc::clone(&self.install);
        cx.spawn(async move |this, cx| {
            let result = cx.background_spawn(async move { install() }).await;
            this.update(cx, |this, cx| match result {
                Ok(()) => cx.quit(),
                Err(error) => {
                    this.progress = Progress::Failed(error);
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
    }

    pub(crate) fn skip(&self) {
        (self.skip)(&self.release.version);
    }
}

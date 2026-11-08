use std::rc::Rc;
use std::sync::Arc;

use domain::settings::Settings;
use gpui_kit::{
    AnyWindowHandle, App, Bounds, Context, WindowBounds, WindowOptions, point, prelude::*, px, size,
};

use crate::native_window::{Spot, handle_of, show_at};
use crate::placement::corner_at;
use crate::settings_logic::{self, Feature};
use crate::theme;

pub struct SettingsActions {
    pub settings: Settings,
    pub enabled: bool,
    pub save: Rc<dyn Fn(Settings)>,
    pub running_programs: Arc<dyn Fn() -> Vec<String>>,
}

pub struct SettingsView {
    pub(crate) settings: Settings,
    pub(crate) enabled: bool,
    pub(crate) picker_open: bool,
    save: Rc<dyn Fn(Settings)>,
    running_programs: Arc<dyn Fn() -> Vec<String>>,
}

pub fn open(cx: &mut App, actions: SettingsActions, at: (i32, i32)) -> Option<AnyWindowHandle> {
    let logical_size = (theme::WINDOW_WIDTH, theme::WINDOW_HEIGHT);
    let bounds = Bounds::new(
        point(px(0.), px(0.)),
        size(px(logical_size.0), px(logical_size.1)),
    );
    let options = WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        titlebar: None,
        is_resizable: false,
        show: true,
        ..Default::default()
    };
    let handle = cx
        .open_window(options, |_, cx| cx.new(|_| SettingsView::new(actions)))
        .ok()?;
    let spot = Spot {
        at,
        logical_size,
        origin: corner_at,
        activate: true,
    };
    cx.spawn(async move |cx| {
        if let Ok(Some(hwnd)) = handle.update(cx, |_, window, _| handle_of(window)) {
            show_at(hwnd, &spot);
        }
    })
    .detach();
    Some(handle.into())
}

impl SettingsView {
    fn new(actions: SettingsActions) -> Self {
        Self {
            settings: actions.settings,
            enabled: actions.enabled,
            picker_open: false,
            save: actions.save,
            running_programs: actions.running_programs,
        }
    }

    pub(crate) fn pickable(&self) -> Vec<String> {
        settings_logic::pickable_programs((self.running_programs)(), &self.settings.excluded)
    }

    pub(crate) fn toggle_feature(&mut self, feature: Feature, cx: &mut Context<Self>) {
        self.change(
            |settings| settings_logic::apply_toggle(settings, feature),
            cx,
        );
    }

    pub(crate) fn remove_program(&mut self, name: &str, cx: &mut Context<Self>) {
        self.change(|settings| settings.excluded.remove(name), cx);
    }

    pub(crate) fn add_program(&mut self, name: &str, cx: &mut Context<Self>) {
        self.picker_open = false;
        self.change(
            |settings| {
                settings.excluded.add(name);
            },
            cx,
        );
    }

    pub(crate) fn toggle_picker(&mut self, cx: &mut Context<Self>) {
        self.picker_open = !self.picker_open;
        cx.notify();
    }

    fn change(&mut self, change: impl FnOnce(&mut Settings), cx: &mut Context<Self>) {
        change(&mut self.settings);
        (self.save)(self.settings.clone());
        cx.notify();
    }
}

use std::rc::Rc;
use std::sync::Arc;

use domain::program::Program;
use domain::settings::Settings;
use gpui_kit::{
    AnyWindowHandle, App, Bounds, Context, WindowBounds, WindowOptions, open_window, point,
    prelude::*, px, size,
};

use crate::native_window::{Spot, handle_of, show_at};
use crate::placement::corner_at;
use crate::settings_logic::{self, Feature};
use crate::theme;

pub struct SettingsActions {
    pub settings: Settings,
    pub enabled: bool,
    pub save: Rc<dyn Fn(Settings)>,
    pub running_programs: Arc<dyn Fn() -> Vec<Program>>,
    pub pick_program: Arc<dyn Fn() -> Option<Program> + Send + Sync>,
}

pub struct SettingsView {
    pub(crate) settings: Settings,
    pub(crate) enabled: bool,
    pub(crate) picker_open: bool,
    pub(crate) picker_toggles: usize,
    programs: Vec<Program>,
    save: Rc<dyn Fn(Settings)>,
    running_programs: Arc<dyn Fn() -> Vec<Program>>,
    pick_program: Arc<dyn Fn() -> Option<Program> + Send + Sync>,
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
    let (handle, _) =
        open_window(options, cx, |_, cx| cx.new(|_| SettingsView::new(actions))).ok()?;
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
    Some(handle)
}

impl SettingsView {
    fn new(actions: SettingsActions) -> Self {
        let programs = (actions.running_programs)();
        Self {
            programs,
            settings: actions.settings,
            enabled: actions.enabled,
            picker_open: false,
            picker_toggles: 0,
            save: actions.save,
            running_programs: actions.running_programs,
            pick_program: actions.pick_program,
        }
    }

    pub(crate) fn pickable(&self) -> Vec<Program> {
        settings_logic::pickable_programs(self.programs.clone(), &self.settings.excluded)
    }

    /// Excluded programs, titled whenever a running or browsed program told us their name.
    pub(crate) fn excluded_programs(&self) -> Vec<Program> {
        self.settings
            .excluded
            .iter()
            .map(|exe| {
                let known = self
                    .programs
                    .iter()
                    .find(|p| p.exe.eq_ignore_ascii_case(exe));
                known.cloned().unwrap_or_else(|| Program::bare(exe))
            })
            .collect()
    }

    fn remember(&mut self, program: Program) {
        match self.programs.iter_mut().find(|p| p.exe == program.exe) {
            Some(known) => *known = program,
            None => self.programs.push(program),
        }
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

    pub(crate) fn browse_program(&mut self, cx: &mut Context<Self>) {
        // The dialog runs its own message loop; on the UI thread it re-enters gpui while the click is still being handled.
        let pick = Arc::clone(&self.pick_program);
        cx.spawn(async move |this, cx| {
            let picked = cx.background_spawn(async move { pick() }).await;
            if let Some(program) = picked {
                this.update(cx, |this, cx| {
                    let exe = program.exe.clone();
                    this.remember(program);
                    this.add_program(&exe, cx);
                })
                .ok();
            }
        })
        .detach();
    }

    pub(crate) fn toggle_picker(&mut self, cx: &mut Context<Self>) {
        self.picker_open = !self.picker_open;
        if self.picker_open {
            for program in (self.running_programs)() {
                self.remember(program);
            }
        }
        self.picker_toggles += 1;
        cx.notify();
    }

    fn change(&mut self, change: impl FnOnce(&mut Settings), cx: &mut Context<Self>) {
        change(&mut self.settings);
        (self.save)(self.settings.clone());
        cx.notify();
    }
}

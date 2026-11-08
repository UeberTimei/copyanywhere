use gpui_kit::component::native_menu::NativeMenu;
use gpui_kit::{AnyWindowHandle, App, actions, point, px};

use crate::native_window::{client_origin, handle_of};
use crate::tray::TrayCommand;

actions!(tray_menu, [ToggleEnabled, OpenSettings, Quit]);

/// Forwards the menu's actions as tray commands.
pub fn on_command(cx: &mut App, send: impl Fn(TrayCommand) + Clone + 'static) {
    let toggle = send.clone();
    cx.on_action(move |_: &ToggleEnabled, _| toggle(TrayCommand::Toggle));
    let settings = send.clone();
    cx.on_action(move |_: &OpenSettings, _| settings(TrayCommand::Settings));
    cx.on_action(move |_: &Quit, _| send(TrayCommand::Exit));
}

/// Pops the OS menu at a screen position (physical pixels), owned by the hidden anchor window.
pub fn show(cx: &mut App, anchor: AnyWindowHandle, at: (i32, i32), enabled: bool) {
    anchor
        .update(cx, |_, window, cx| {
            // The menu is placed relative to the anchor's client area, wherever GPUI put it.
            let origin = handle_of(window).map_or((0, 0), client_origin);
            let scale = window.scale_factor();
            let at = point(
                px((at.0 - origin.0) as f32 / scale),
                px((at.1 - origin.1) as f32 / scale),
            );
            NativeMenu::new()
                .menu(toggle_label(enabled), Box::new(ToggleEnabled))
                .menu("Settings", Box::new(OpenSettings))
                .separator()
                .menu("Exit", Box::new(Quit))
                .show(at, window, cx);
        })
        .ok();
}

pub fn toggle_label(enabled: bool) -> &'static str {
    if enabled { "Disable" } else { "Enable" }
}

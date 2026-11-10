use tray_icon::menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem};

use crate::tray::{TrayCommand, TrayError};
use crate::tray_menu::toggle_label;

const TOGGLE: &str = "toggle";
const SETTINGS: &str = "settings";
const EXIT: &str = "exit";

/// Status notifier hosts show no click events, so Linux gets the menu on the icon itself.
pub struct LinuxMenu {
    pub menu: Menu,
    toggle: MenuItem,
}

impl LinuxMenu {
    pub fn new(enabled: bool) -> Result<LinuxMenu, TrayError> {
        let toggle = MenuItem::with_id(TOGGLE, toggle_label(enabled), true, None);
        let menu = Menu::new();
        menu.append_items(&[
            &toggle,
            &MenuItem::with_id(SETTINGS, "Settings", true, None),
            &PredefinedMenuItem::separator(),
            &MenuItem::with_id(EXIT, "Exit", true, None),
        ])?;
        Ok(LinuxMenu { menu, toggle })
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.toggle.set_text(toggle_label(enabled));
    }

    pub fn poll() -> Option<TrayCommand> {
        MenuEvent::receiver()
            .try_iter()
            .find_map(|event| command(event.id.as_ref()))
    }
}

fn command(id: &str) -> Option<TrayCommand> {
    match id {
        TOGGLE => Some(TrayCommand::Toggle),
        SETTINGS => Some(TrayCommand::Settings),
        EXIT => Some(TrayCommand::Exit),
        _ => None,
    }
}

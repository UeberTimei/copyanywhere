use tray_icon::menu::{Menu, MenuEvent, MenuItem};
use tray_icon::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent};

use crate::tray_icon_image;

const TOOLTIP: &str = "CopyAnywhere";
const TOGGLE_ID: &str = "toggle";
const SETTINGS_ID: &str = "settings";
const EXIT_ID: &str = "exit";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayCommand {
    Toggle,
    Settings,
    Exit,
}

#[derive(Debug, thiserror::Error)]
pub enum TrayError {
    #[error("tray icon failed: {0}")]
    TrayIcon(#[from] tray_icon::Error),
    #[error("tray icon image is invalid: {0}")]
    Image(#[from] tray_icon::BadIcon),
    #[error("tray menu failed: {0}")]
    Menu(#[from] tray_icon::menu::Error),
}

pub struct Tray {
    icon: TrayIcon,
    toggle_item: MenuItem,
}

impl Tray {
    pub fn new(enabled: bool) -> Result<Tray, TrayError> {
        let toggle_item = MenuItem::with_id(TOGGLE_ID, toggle_label(enabled), true, None);
        let menu = Menu::new();
        menu.append(&toggle_item)?;
        menu.append(&MenuItem::with_id(SETTINGS_ID, "Settings", true, None))?;
        menu.append(&MenuItem::with_id(EXIT_ID, "Exit", true, None))?;
        let icon = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_menu_on_left_click(false)
            .with_tooltip(TOOLTIP)
            .with_icon(tray_icon_image::icon(enabled)?)
            .build()?;
        Ok(Tray { icon, toggle_item })
    }

    pub fn set_enabled(&self, enabled: bool) -> Result<(), TrayError> {
        self.toggle_item.set_text(toggle_label(enabled));
        self.icon.set_icon(Some(tray_icon_image::icon(enabled)?))?;
        Ok(())
    }

    pub fn poll(&self) -> Option<TrayCommand> {
        let clicks = TrayIconEvent::receiver()
            .try_iter()
            .filter_map(|event| match event {
                TrayIconEvent::Click {
                    button,
                    button_state,
                    ..
                } => click_command(button, button_state),
                _ => None,
            });
        let menus = MenuEvent::receiver()
            .try_iter()
            .filter_map(|event| menu_command(event.id.as_ref()));
        clicks.chain(menus).next()
    }
}

pub fn menu_command(id: &str) -> Option<TrayCommand> {
    match id {
        TOGGLE_ID => Some(TrayCommand::Toggle),
        SETTINGS_ID => Some(TrayCommand::Settings),
        EXIT_ID => Some(TrayCommand::Exit),
        _ => None,
    }
}

pub fn click_command(button: MouseButton, state: MouseButtonState) -> Option<TrayCommand> {
    (button == MouseButton::Left && state == MouseButtonState::Up).then_some(TrayCommand::Toggle)
}

pub fn toggle_label(enabled: bool) -> &'static str {
    if enabled { "Disable" } else { "Enable" }
}

#[cfg(test)]
mod tests;

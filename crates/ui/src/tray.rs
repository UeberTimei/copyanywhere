use tray_icon::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent};

use crate::tray_icon_image;
#[cfg(target_os = "linux")]
use crate::tray_linux_menu::LinuxMenu;

const TOOLTIP: &str = "CopyAnywhere";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayCommand {
    Toggle,
    Settings,
    Exit,
    Menu,
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
    #[cfg(target_os = "linux")]
    menu: LinuxMenu,
}

impl Tray {
    pub fn new(enabled: bool) -> Result<Tray, TrayError> {
        let builder = TrayIconBuilder::new()
            .with_tooltip(TOOLTIP)
            .with_icon(tray_icon_image::icon(enabled)?);
        #[cfg(target_os = "linux")]
        let menu = LinuxMenu::new(enabled)?;
        #[cfg(target_os = "linux")]
        let builder = builder.with_menu(Box::new(menu.menu.clone()));
        let icon = builder.build()?;
        Ok(Tray {
            icon,
            #[cfg(target_os = "linux")]
            menu,
        })
    }

    pub fn set_enabled(&self, enabled: bool) -> Result<(), TrayError> {
        self.icon.set_icon(Some(tray_icon_image::icon(enabled)?))?;
        #[cfg(target_os = "linux")]
        self.menu.set_enabled(enabled);
        Ok(())
    }

    pub fn poll(&self) -> Option<TrayCommand> {
        #[cfg(target_os = "linux")]
        if let Some(command) = LinuxMenu::poll() {
            return Some(command);
        }
        TrayIconEvent::receiver()
            .try_iter()
            .find_map(|event| match event {
                TrayIconEvent::Click {
                    button,
                    button_state,
                    ..
                } => click_command(button, button_state),
                _ => None,
            })
    }
}

pub fn click_command(button: MouseButton, state: MouseButtonState) -> Option<TrayCommand> {
    if state != MouseButtonState::Up {
        return None;
    }
    match button {
        MouseButton::Left => Some(TrayCommand::Toggle),
        MouseButton::Right => Some(TrayCommand::Menu),
        _ => None,
    }
}

#[cfg(test)]
mod tests;

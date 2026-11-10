use std::rc::Rc;
use std::sync::Arc;

use application::session::Session;
use domain::settings::Settings;
use domain::update::Release;
use infrastructure::{autostart, file_dialog, foreground, self_update, settings_file};
use ui::settings_window::SettingsActions;
use ui::update_window::UpdateActions;

pub fn settings_actions(session: &Arc<Session>) -> SettingsActions {
    let target = Arc::clone(session);
    SettingsActions {
        settings: session.settings(),
        enabled: session.is_enabled(),
        save: Rc::new(move |next| persist(&target, next)),
        running_programs: Arc::new(foreground::running_programs),
        pick_program: Arc::new(file_dialog::pick_program),
    }
}

pub fn update_actions(session: &Arc<Session>, release: Release) -> UpdateActions {
    let target = Arc::clone(session);
    let download = release.clone();
    UpdateActions {
        release,
        install: Arc::new(move || self_update::install(&download).map_err(|e| e.to_string())),
        skip: Rc::new(move |version| {
            let skipped = version.to_owned();
            persist(
                &target,
                Settings {
                    skipped_version: skipped,
                    ..target.settings()
                },
            );
        }),
    }
}

fn persist(session: &Session, next: Settings) {
    if next.launch_at_login != session.settings().launch_at_login {
        autostart::set(next.launch_at_login).ok();
    }
    settings_file::save(&next).ok();
    session.update(|current| *current = next);
}

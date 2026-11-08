use std::rc::Rc;
use std::sync::Arc;

use application::session::Session;
use domain::settings::Settings;
use infrastructure::{autostart, file_dialog, foreground, settings_file};
use ui::settings_window::SettingsActions;

pub fn settings_actions(session: &Arc<Session>) -> SettingsActions {
    let target = Arc::clone(session);
    SettingsActions {
        settings: session.settings(),
        enabled: session.is_enabled(),
        save: Rc::new(move |next| persist(&target, next)),
        running_programs: Arc::new(foreground::running_programs),
        pick_program: Rc::new(file_dialog::pick_program),
    }
}

fn persist(session: &Session, next: Settings) {
    if next.launch_at_login != session.settings().launch_at_login {
        autostart::set(next.launch_at_login).ok();
    }
    settings_file::save(&next).ok();
    session.update(|current| *current = next);
}

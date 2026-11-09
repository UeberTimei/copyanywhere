#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod actions;
mod adapters;
mod pipeline;
mod shell;

use std::sync::Arc;
use std::sync::mpsc;

use application::auto_copy::AutoCopy;
use application::session::Session;
use domain::update::is_offered;
use infrastructure::{release_feed, self_update, settings_file};
use ui::tray::Tray;

fn main() -> anyhow::Result<()> {
    let session = Arc::new(Session::new(settings_file::load()));
    let (toasts, toast_requests) = mpsc::channel();
    let feedback = adapters::ToastFeedback { toasts };
    let auto_copy = AutoCopy::new(
        adapters::SystemClipboard,
        adapters::SystemForeground,
        feedback,
        Arc::clone(&session),
    );
    pipeline::start(auto_copy);
    self_update::remove_leftover();
    let (releases, updates) = mpsc::channel();
    let settings = session.settings();
    if settings.check_updates {
        std::thread::spawn(move || {
            let offered = release_feed::latest()
                .ok()
                .filter(|r| is_offered(r, env!("CARGO_PKG_VERSION"), &settings.skipped_version));
            if let Some(release) = offered {
                releases.send(release).ok();
            }
        });
    }
    let tray = Tray::new(session.is_enabled())?;
    shell::run(session, tray, toast_requests, updates);
    Ok(())
}

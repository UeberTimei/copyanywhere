#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod actions;
mod adapters;
mod pipeline;
mod shell;

use std::sync::Arc;
use std::sync::mpsc;

use application::auto_copy::AutoCopy;
use application::session::Session;
use infrastructure::settings_file;
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
    let tray = Tray::new(session.is_enabled())?;
    shell::run(session, tray, toast_requests);
    Ok(())
}

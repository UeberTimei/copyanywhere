use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::time::Duration;

use application::session::Session;
use gpui_kit::{AnyWindowHandle, App, application};
use infrastructure::mouse_hook;
use ui::tray::{Tray, TrayCommand};
use ui::{anchor, settings_window, toast};

use crate::actions::settings_actions;

const TICK: Duration = Duration::from_millis(16);

pub fn run(session: Arc<Session>, tray: Tray, toasts: Receiver<(i32, i32)>) {
    application().run(move |cx: &mut App| {
        anchor::open(cx);
        let mut window: Option<AnyWindowHandle> = None;
        cx.spawn(async move |cx| {
            loop {
                cx.background_executor().timer(TICK).await;
                cx.update(|cx| {
                    while let Ok(at) = toasts.try_recv() {
                        toast::show(cx, at);
                    }
                    while let Some(command) = tray.poll() {
                        handle(command, &session, &tray, &mut window, cx);
                    }
                });
            }
        })
        .detach();
    });
}

fn handle(
    command: TrayCommand,
    session: &Arc<Session>,
    tray: &Tray,
    window: &mut Option<AnyWindowHandle>,
    cx: &mut App,
) {
    match command {
        TrayCommand::Toggle => {
            tray.set_enabled(session.toggle()).ok();
        }
        TrayCommand::Settings => open_settings(session, window, cx),
        TrayCommand::Exit => cx.quit(),
    }
}

fn open_settings(session: &Arc<Session>, window: &mut Option<AnyWindowHandle>, cx: &mut App) {
    let open = window.filter(|handle| cx.windows().contains(handle));
    match open {
        Some(handle) => {
            handle
                .update(cx, |_, window, _| window.activate_window())
                .ok();
        }
        None => {
            *window =
                settings_window::open(cx, settings_actions(session), mouse_hook::cursor_position())
        }
    }
}

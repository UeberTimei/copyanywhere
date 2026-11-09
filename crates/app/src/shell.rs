use std::sync::Arc;
use std::sync::mpsc::{self, Receiver};
use std::time::Duration;

use application::session::Session;
use gpui_kit::component::theme::{Theme, ThemeMode};
use gpui_kit::{AnyWindowHandle, App, QuitMode, application, assets, init};
use infrastructure::mouse_hook;
use ui::tray::{Tray, TrayCommand};
use ui::{anchor, settings_window, toast, tray_menu};

use domain::update::Release;
use ui::update_window;

use crate::actions::{settings_actions, update_actions};

/// Lightness added to the Kit's dark palette.
const LIFT: f32 = 0.05;
const TICK: Duration = Duration::from_millis(16);

pub fn run(
    session: Arc<Session>,
    tray: Tray,
    toasts: Receiver<(i32, i32)>,
    updates: Receiver<Release>,
) {
    application()
        .with_assets(assets::Assets)
        // the tray, not a window, keeps the app alive
        .with_quit_mode(QuitMode::Explicit)
        .run(move |cx: &mut App| {
            init(cx);
            Theme::change(ThemeMode::Dark, None, cx);
            Theme::update(cx, |theme| {
                theme.background.l += LIFT;
                theme.group_box.l += LIFT;
            });
            let (commands, menu_commands) = mpsc::channel();
            tray_menu::on_command(cx, move |command| {
                commands.send(command).ok();
            });
            let mut window: Option<AnyWindowHandle> = None;
            // opened by the first right-click, since only the native menu needs it
            let mut anchor: Option<AnyWindowHandle> = None;
            cx.spawn(async move |cx| {
                loop {
                    cx.background_executor().timer(TICK).await;
                    cx.update(|cx| {
                        while let Ok(at) = toasts.try_recv() {
                            toast::show(cx, at);
                        }
                        while let Ok(release) = updates.try_recv() {
                            update_window::open(
                                cx,
                                update_actions(&session, release),
                                mouse_hook::cursor_position(),
                            );
                        }
                        while let Some(command) =
                            tray.poll().or_else(|| menu_commands.try_recv().ok())
                        {
                            if command == TrayCommand::Menu {
                                if anchor.is_none() {
                                    anchor = anchor::open(cx);
                                }
                                if let Some(anchor) = anchor {
                                    let at = mouse_hook::cursor_position();
                                    tray_menu::show(cx, anchor, at, session.is_enabled());
                                }
                                continue;
                            }
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
        TrayCommand::Menu => {}
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

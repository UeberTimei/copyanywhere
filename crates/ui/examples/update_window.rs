//! Shows the update window with made-up release notes: `cargo xtask update-example`.
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use domain::update::Release;
use gpui_kit::component::theme::{Theme, ThemeMode};
use gpui_kit::{App, application, assets, init};
use ui::update_window::{UpdateActions, open};

const LIFT: f32 = 0.05;
const NOTES: &str = "## What's Changed\n* feat: check for updates on launch by @UeberTimei\n* fix: load static imports from System32 only by @UeberTimei\n* ci: publish a SHA-256 checksum next to the release exe by @UeberTimei\n* docs: explain SmartScreen and antivirus warnings in the README by @UeberTimei\n* build: shrink the release exe with lto and strip by @UeberTimei\n\n**Full Changelog**: https://github.com/UeberTimei/copyanywhere/compare/v1.0.0...v1.1.0";

fn main() {
    application()
        .with_assets(assets::Assets)
        .run(|cx: &mut App| {
            init(cx);
            Theme::change(ThemeMode::Dark, None, cx);
            Theme::update(cx, |theme| {
                theme.background.l += LIFT;
                theme.group_box.l += LIFT;
            });
            cx.on_window_closed(|cx, _| {
                if cx.windows().is_empty() {
                    cx.quit();
                }
            })
            .detach();
            let actions = UpdateActions {
                release: Release {
                    version: "v1.1.0".into(),
                    notes: NOTES.into(),
                    exe_url: String::new(),
                    checksum_url: None,
                },
                install: Arc::new(|| {
                    std::thread::sleep(Duration::from_secs(2));
                    Err("this is only a preview, nothing was installed".into())
                }),
                skip: Rc::new(|_| {}),
            };
            open(cx, actions, (0, 0));
        });
}

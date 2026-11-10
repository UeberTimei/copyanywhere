use gpui_kit::{App, Pixels, Point, Window, point, px};

use super::Spot;
use crate::placement::Area;

/// Stands in for the native window handle: GPUI places windows itself here.
pub struct Handle;

pub fn handle_of(_: &Window) -> Option<Handle> {
    Some(Handle)
}

pub fn show_at(_: Handle, _: &Spot) {}

/// The menu is placed against the anchor window, which opens at the screen origin.
pub fn client_origin(_: Handle) -> (i32, i32) {
    (0, 0)
}

/// Cursor coordinates are already logical here.
pub fn cursor_scale(_: &Window) -> f32 {
    1.0
}

/// Where the window opens: GPUI cannot move a window once it exists, so it is placed up front.
pub fn opening_origin(cx: &App, spot: &Spot) -> Point<Pixels> {
    let screen = cx
        .displays()
        .iter()
        .map(|display| {
            let bounds = display.bounds();
            let (left, top) = (f32::from(bounds.origin.x), f32::from(bounds.origin.y));
            Area {
                left: left as i32,
                top: top as i32,
                right: (left + f32::from(bounds.size.width)) as i32,
                bottom: (top + f32::from(bounds.size.height)) as i32,
            }
        })
        .find(|area| holds(area, spot.at))
        .unwrap_or(Area {
            left: i32::MIN / 2,
            top: i32::MIN / 2,
            right: i32::MAX / 2,
            bottom: i32::MAX / 2,
        });
    let size = (
        spot.logical_size.0.round() as i32,
        spot.logical_size.1.round() as i32,
    );
    let (x, y) = (spot.origin)(spot.at, size, screen);
    point(px(x as f32), px(y as f32))
}

fn holds(area: &Area, at: (i32, i32)) -> bool {
    (area.left..area.right).contains(&at.0) && (area.top..area.bottom).contains(&at.1)
}

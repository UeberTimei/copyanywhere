pub const TOAST_SIZE: (f32, f32) = (96.0, 32.0);

const CURSOR_OFFSET: i32 = 12;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Area {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

pub fn pixel_size(dpi: u32) -> (i32, i32) {
    let scale = dpi as f32 / 96.0;
    (
        (TOAST_SIZE.0 * scale).round() as i32,
        (TOAST_SIZE.1 * scale).round() as i32,
    )
}

pub fn popup_origin(at: (i32, i32), size: (i32, i32), screen: Area) -> (i32, i32) {
    let above = at.1 - size.1 - CURSOR_OFFSET;
    let y = if above >= screen.top {
        above
    } else {
        at.1 + CURSOR_OFFSET
    };
    let x = (at.0 + CURSOR_OFFSET).min(screen.right - size.0);
    (x.max(screen.left), y.min(screen.bottom - size.1))
}

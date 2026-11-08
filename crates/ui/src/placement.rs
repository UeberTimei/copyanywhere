#[cfg(test)]
mod tests;

const CURSOR_OFFSET: i32 = 12;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Area {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

pub type Origin = fn((i32, i32), (i32, i32), Area) -> (i32, i32);

pub fn pixel_size(logical: (f32, f32), dpi: u32) -> (i32, i32) {
    let scale = dpi as f32 / 96.0;
    (
        (logical.0 * scale).round() as i32,
        (logical.1 * scale).round() as i32,
    )
}

pub fn above_right(at: (i32, i32), size: (i32, i32), screen: Area) -> (i32, i32) {
    let above = at.1 - size.1 - CURSOR_OFFSET;
    let y = if above >= screen.top {
        above
    } else {
        at.1 + CURSOR_OFFSET
    };
    inside(at.0 + CURSOR_OFFSET, y, size, screen)
}

pub fn centered_above(at: (i32, i32), size: (i32, i32), screen: Area) -> (i32, i32) {
    let above = at.1 - size.1 - CURSOR_OFFSET;
    let y = if above >= screen.top {
        above
    } else {
        at.1 + CURSOR_OFFSET
    };
    inside(at.0 - size.0 / 2, y, size, screen)
}

fn inside(x: i32, y: i32, size: (i32, i32), screen: Area) -> (i32, i32) {
    (
        x.min(screen.right - size.0).max(screen.left),
        y.min(screen.bottom - size.1).max(screen.top),
    )
}

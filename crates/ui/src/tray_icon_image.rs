use tray_icon::{BadIcon, Icon};

const SIZE: u32 = 32;
const CENTRE: f32 = SIZE as f32 / 2.0;
const SHEET: f32 = 8.0; // half the side of one sheet
const OFFSET: f32 = 5.0; // how far the front sheet sits down-right of the middle
const CORNER: f32 = 3.0;
const HALF_STROKE: f32 = 1.25;
const GAP: f32 = 2.0; // clear space where the front sheet hides the back one
const ACCENT: [u8; 3] = [0x46, 0xc2, 0xa1];
const MUTED: [u8; 3] = [0xe5, 0x48, 0x4d];

// Signed distance to a rounded square centred at (cx, cy); negative inside.
fn distance(x: f32, y: f32, cx: f32, cy: f32) -> f32 {
    let inner = SHEET - CORNER;
    let dx = (x - cx).abs() - inner;
    let dy = (y - cy).abs() - inner;
    dx.max(0.0).hypot(dy.max(0.0)) + dx.max(dy).min(0.0) - CORNER
}

fn coverage(value: f32) -> f32 {
    (value + 0.5).clamp(0.0, 1.0)
}

/// Two overlapping sheets: the universal "copy" glyph.
pub fn rgba(enabled: bool) -> Vec<u8> {
    let [red, green, blue] = if enabled { ACCENT } else { MUTED };
    (0..SIZE * SIZE)
        .flat_map(|index| {
            let x = (index % SIZE) as f32 + 0.5;
            let y = (index / SIZE) as f32 + 0.5;
            let front = distance(x, y, CENTRE + OFFSET, CENTRE + OFFSET);
            let back = distance(x, y, CENTRE - OFFSET, CENTRE - OFFSET);
            let hidden = coverage(front - GAP);
            let alpha = coverage(HALF_STROKE - front.abs())
                .max(coverage(HALF_STROKE - back.abs()) * hidden);
            [red, green, blue, (alpha * 255.0).round() as u8]
        })
        .collect()
}

pub fn icon(enabled: bool) -> Result<Icon, BadIcon> {
    Icon::from_rgba(rgba(enabled), SIZE, SIZE)
}

#[cfg(test)]
mod tests;

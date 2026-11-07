use tray_icon::{BadIcon, Icon};

const SIZE: u32 = 32;
const CENTRE: f32 = SIZE as f32 / 2.0;
const RADIUS: f32 = 14.0;
const ACCENT: [u8; 3] = [0x46, 0xc2, 0xa1];
const MUTED: [u8; 3] = [0x85, 0x85, 0x8b];

pub fn rgba(enabled: bool) -> Vec<u8> {
    let [red, green, blue] = if enabled { ACCENT } else { MUTED };
    (0..SIZE * SIZE)
        .flat_map(|index| {
            let x = (index % SIZE) as f32 + 0.5;
            let y = (index / SIZE) as f32 + 0.5;
            let distance = (x - CENTRE).hypot(y - CENTRE);
            let coverage = (RADIUS - distance + 0.5).clamp(0.0, 1.0);
            [red, green, blue, (coverage * 255.0).round() as u8]
        })
        .collect()
}

pub fn icon(enabled: bool) -> Result<Icon, BadIcon> {
    Icon::from_rgba(rgba(enabled), SIZE, SIZE)
}

#[cfg(test)]
mod tests;

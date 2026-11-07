pub const TOAST_SIZE: (f32, f32) = (96.0, 32.0);

const CURSOR_OFFSET: f32 = 16.0;

pub fn popup_origin(at: (i32, i32), scale: f32) -> (f32, f32) {
    (
        at.0 as f32 / scale + CURSOR_OFFSET,
        at.1 as f32 / scale + CURSOR_OFFSET,
    )
}

pub fn dpi_scale(dpi: u32) -> f32 {
    dpi as f32 / 96.0
}

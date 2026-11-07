use super::geometry::dpi_scale;

#[cfg(windows)]
#[link(name = "user32")]
unsafe extern "system" {
    fn GetDpiForSystem() -> u32;
}

#[cfg(windows)]
pub fn system_scale() -> f32 {
    // shortcut: system DPI of the primary monitor only, mixed-DPI setups
    // are off by the ratio of the other monitor; upgrade via GetDpiForMonitor
    dpi_scale(unsafe { GetDpiForSystem() })
}

#[cfg(not(windows))]
pub fn system_scale() -> f32 {
    dpi_scale(96)
}

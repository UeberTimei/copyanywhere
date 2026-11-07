use gpui::Window;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use windows::Win32::Foundation::{HWND, POINT};
use windows::Win32::Graphics::Gdi::{
    GetMonitorInfoW, MONITOR_DEFAULTTONEAREST, MONITORINFO, MonitorFromPoint,
};
use windows::Win32::UI::HiDpi::GetDpiForWindow;
use windows::Win32::UI::WindowsAndMessaging::{
    GWL_STYLE, HWND_TOPMOST, SET_WINDOW_POS_FLAGS, SWP_FRAMECHANGED, SWP_NOACTIVATE, SWP_NOSIZE,
    SWP_NOZORDER, SWP_SHOWWINDOW, SetWindowLongPtrW, SetWindowPos, WS_POPUP,
};

use crate::placement::{Area, Origin, pixel_size};

pub struct Spot {
    pub at: (i32, i32),
    pub logical_size: (f32, f32),
    pub origin: Origin,
    pub activate: bool,
}

// GPUI creates popups with an overlapped style whose minimum height includes a caption, so
// the window is turned into a plain popup and given its exact size and place natively.
// Must run outside an app update: resizing makes GPUI paint synchronously
pub fn show_at(hwnd: HWND, spot: &Spot) {
    let screen = monitor_area(spot.at);
    let keep_focus = if spot.activate {
        SET_WINDOW_POS_FLAGS(0)
    } else {
        SWP_NOACTIVATE
    };
    unsafe {
        SetWindowLongPtrW(hwnd, GWL_STYLE, WS_POPUP.0 as isize);
        let _ = SetWindowPos(
            hwnd,
            None,
            spot.at.0,
            spot.at.1,
            0,
            0,
            SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE | SWP_FRAMECHANGED,
        );
        let size = pixel_size(spot.logical_size, GetDpiForWindow(hwnd));
        let (x, y) = (spot.origin)(spot.at, size, screen);
        let _ = SetWindowPos(
            hwnd,
            Some(HWND_TOPMOST),
            x,
            y,
            size.0,
            size.1,
            keep_focus | SWP_SHOWWINDOW,
        );
    }
}

pub fn handle_of(window: &Window) -> Option<HWND> {
    match HasWindowHandle::window_handle(window).ok()?.as_raw() {
        RawWindowHandle::Win32(handle) => Some(HWND(handle.hwnd.get() as *mut _)),
        _ => None,
    }
}

fn monitor_area(at: (i32, i32)) -> Area {
    let mut info = MONITORINFO {
        cbSize: size_of::<MONITORINFO>() as u32,
        ..Default::default()
    };
    let monitor = unsafe { MonitorFromPoint(POINT { x: at.0, y: at.1 }, MONITOR_DEFAULTTONEAREST) };
    if !unsafe { GetMonitorInfoW(monitor, &mut info) }.as_bool() {
        return Area {
            left: i32::MIN / 2,
            top: i32::MIN / 2,
            right: i32::MAX / 2,
            bottom: i32::MAX / 2,
        };
    }
    let work = info.rcWork;
    Area {
        left: work.left,
        top: work.top,
        right: work.right,
        bottom: work.bottom,
    }
}

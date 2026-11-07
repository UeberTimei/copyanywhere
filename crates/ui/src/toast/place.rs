use gpui::Window;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use windows::Win32::Foundation::{HWND, POINT};
use windows::Win32::Graphics::Gdi::{
    GetMonitorInfoW, MONITOR_DEFAULTTONEAREST, MONITORINFO, MonitorFromPoint,
};
use windows::Win32::UI::HiDpi::GetDpiForWindow;
use windows::Win32::UI::WindowsAndMessaging::{
    HWND_TOPMOST, SWP_NOACTIVATE, SWP_NOSIZE, SWP_NOZORDER, SWP_SHOWWINDOW, SetWindowPos,
};

use super::geometry::{Area, pixel_size, popup_origin};

pub fn show_near(window: &Window, at: (i32, i32)) {
    let Some(hwnd) = native_handle(window) else {
        return;
    };
    let screen = monitor_area(at);
    unsafe {
        let _ = SetWindowPos(
            hwnd,
            None,
            at.0,
            at.1,
            0,
            0,
            SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE,
        );
        let size = pixel_size(GetDpiForWindow(hwnd));
        let (x, y) = popup_origin(at, size, screen);
        let _ = SetWindowPos(
            hwnd,
            Some(HWND_TOPMOST),
            x,
            y,
            size.0,
            size.1,
            SWP_NOACTIVATE | SWP_SHOWWINDOW,
        );
    }
}

fn native_handle(window: &Window) -> Option<HWND> {
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

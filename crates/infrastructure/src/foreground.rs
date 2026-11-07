#![allow(dead_code)]

use std::collections::BTreeSet;

use windows::Win32::Foundation::{CloseHandle, HWND, LPARAM};
use windows::Win32::System::Threading::{
    OpenProcess, PROCESS_NAME_FORMAT, PROCESS_QUERY_LIMITED_INFORMATION, QueryFullProcessImageNameW,
};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetForegroundWindow, GetWindowTextLengthW, GetWindowThreadProcessId,
    IsWindowVisible,
};
use windows::core::{BOOL, PWSTR};

#[cfg(test)]
mod tests;

pub fn exe_name(path: &str) -> String {
    path.rsplit(['\\', '/'])
        .next()
        .unwrap_or_default()
        .to_lowercase()
}

pub fn foreground_program() -> Option<String> {
    window_program(unsafe { GetForegroundWindow() })
}

pub fn running_programs() -> Vec<String> {
    let mut handles: Vec<HWND> = Vec::new();
    let lparam = LPARAM(&mut handles as *mut Vec<HWND> as isize);
    let _ = unsafe { EnumWindows(Some(collect_titled_window), lparam) };
    handles
        .into_iter()
        .filter_map(window_program)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

unsafe extern "system" fn collect_titled_window(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let handles = unsafe { &mut *(lparam.0 as *mut Vec<HWND>) };
    let visible = unsafe { IsWindowVisible(hwnd) }.as_bool();
    let titled = unsafe { GetWindowTextLengthW(hwnd) } > 0;
    if visible && titled {
        handles.push(hwnd);
    }
    BOOL(1)
}

fn window_program(hwnd: HWND) -> Option<String> {
    let mut pid = 0u32;
    unsafe { GetWindowThreadProcessId(hwnd, Some(&mut pid)) };
    process_image_path(pid).map(|path| exe_name(&path))
}

fn process_image_path(pid: u32) -> Option<String> {
    // shortcut: image paths longer than 1024 UTF-16 units fail to resolve, raise the buffer if that shows up
    let mut buffer = [0u16; 1024];
    let mut size = buffer.len() as u32;
    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid).ok()?;
        let queried = QueryFullProcessImageNameW(
            handle,
            PROCESS_NAME_FORMAT(0),
            PWSTR(buffer.as_mut_ptr()),
            &mut size,
        );
        let _ = CloseHandle(handle);
        queried.ok()?;
    }
    Some(String::from_utf16_lossy(&buffer[..size as usize]))
}

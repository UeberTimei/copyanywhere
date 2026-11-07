#![allow(dead_code)]

use std::sync::{OnceLock, mpsc::Sender};

use windows::Win32::Foundation::{LPARAM, LRESULT, POINT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, GetCursorPos, GetMessageW, MSG, MSLLHOOKSTRUCT, SetWindowsHookExW, WH_MOUSE_LL,
    WM_LBUTTONDOWN, WM_LBUTTONUP,
};

use crate::domain::selection::MouseEvent;

static SENDER: OnceLock<Sender<MouseEvent>> = OnceLock::new();

pub fn install(tx: Sender<MouseEvent>) {
    let _ = SENDER.set(tx);
    std::thread::spawn(run_message_loop);
}

pub fn cursor_position() -> (i32, i32) {
    let mut point = POINT::default();
    unsafe { GetCursorPos(&mut point) }.map_or((0, 0), |()| (point.x, point.y))
}

fn run_message_loop() {
    let Ok(module) = (unsafe { GetModuleHandleW(None) }) else {
        return;
    };
    if unsafe { SetWindowsHookExW(WH_MOUSE_LL, Some(hook_proc), Some(module.into()), 0) }.is_err() {
        return;
    }
    let mut msg = MSG::default();
    while unsafe { GetMessageW(&mut msg, None, 0, 0) }.0 > 0 {}
}

unsafe extern "system" fn hook_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code >= 0 {
        let info = unsafe { &*(lparam.0 as *const MSLLHOOKSTRUCT) };
        let event = to_event(wparam.0 as u32, info.pt.x, info.pt.y, u64::from(info.time));
        if let (Some(event), Some(tx)) = (event, SENDER.get()) {
            let _ = tx.send(event);
        }
    }
    unsafe { CallNextHookEx(None, code, wparam, lparam) }
}

fn to_event(message: u32, x: i32, y: i32, at_ms: u64) -> Option<MouseEvent> {
    match message {
        WM_LBUTTONDOWN => Some(MouseEvent::Down { x, y, at_ms }),
        WM_LBUTTONUP => Some(MouseEvent::Up { x, y, at_ms }),
        _ => None,
    }
}

#[cfg(test)]
mod tests;

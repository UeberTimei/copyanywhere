use windows::Win32::UI::Controls::Dialogs::{
    GetOpenFileNameW, OFN_EXPLORER, OFN_FILEMUSTEXIST, OFN_NOCHANGEDIR, OPENFILENAMEW,
};
use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;
use windows::core::{PCWSTR, PWSTR};

use crate::foreground::exe_name;

/// Asks the user for an executable and returns its file name, or `None` if cancelled.
pub fn pick_program() -> Option<String> {
    let filter: Vec<u16> = "Programs (*.exe)\0*.exe\0\0".encode_utf16().collect();
    // shortcut: paths longer than 1024 UTF-16 units are rejected by the dialog, raise the buffer if that shows up
    let mut buffer = [0u16; 1024];
    let mut dialog = OPENFILENAMEW {
        lStructSize: size_of::<OPENFILENAMEW>() as u32,
        hwndOwner: unsafe { GetForegroundWindow() },
        lpstrFilter: PCWSTR(filter.as_ptr()),
        lpstrFile: PWSTR(buffer.as_mut_ptr()),
        nMaxFile: buffer.len() as u32,
        Flags: OFN_EXPLORER | OFN_FILEMUSTEXIST | OFN_NOCHANGEDIR,
        ..Default::default()
    };
    if !unsafe { GetOpenFileNameW(&mut dialog) }.as_bool() {
        return None;
    }
    let len = buffer.iter().position(|&unit| unit == 0)?;
    Some(exe_name(&String::from_utf16_lossy(&buffer[..len])))
}

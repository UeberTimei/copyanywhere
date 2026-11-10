use windows::Win32::System::Threading::{GetCurrentProcess, SetProcessWorkingSetSize};

/// Hands idle pages back to Windows; they fault back in when the app touches them again.
pub fn trim() {
    // (usize::MAX, usize::MAX) is the documented "empty the working set" request
    let _ = unsafe { SetProcessWorkingSetSize(GetCurrentProcess(), usize::MAX, usize::MAX) };
}

#[cfg(test)]
mod tests;

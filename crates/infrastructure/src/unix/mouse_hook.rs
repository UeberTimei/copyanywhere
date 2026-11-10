use std::sync::mpsc::Sender;
use std::thread::sleep;
use std::time::{Duration, Instant};

use device_query::{DeviceQuery, DeviceState};
use domain::selection::MouseEvent;

#[cfg(test)]
mod tests;

const POLL: Duration = Duration::from_millis(10);
/// `button_pressed` is 1-based: 1 is the left button.
const LEFT: usize = 1;

/// Polls the pointer instead of hooking it, which needs no permission on macOS or X11.
pub fn install(tx: Sender<MouseEvent>) {
    std::thread::spawn(move || {
        let Some(device) = device_state() else {
            return;
        };
        let started = Instant::now();
        let mut was_down = false;
        loop {
            sleep(POLL);
            let mouse = device.get_mouse();
            let down = mouse.button_pressed.get(LEFT).copied().unwrap_or(false);
            let (x, y) = mouse.coords;
            let at_ms = started.elapsed().as_millis() as u64;
            if let Some(event) = edge(was_down, down, (x, y), at_ms) {
                let _ = tx.send(event);
            }
            was_down = down;
        }
    });
}

pub fn cursor_position() -> (i32, i32) {
    device_state().map_or((0, 0), |device| device.get_mouse().coords)
}

// DeviceState panics on Linux when there is no X display, and the release build aborts on panic
fn device_state() -> Option<DeviceState> {
    if cfg!(target_os = "linux") && std::env::var_os("DISPLAY").is_none() {
        return None;
    }
    Some(DeviceState::new())
}

fn edge(was_down: bool, down: bool, at: (i32, i32), at_ms: u64) -> Option<MouseEvent> {
    let (x, y) = at;
    match (was_down, down) {
        (false, true) => Some(MouseEvent::Down { x, y, at_ms }),
        (true, false) => Some(MouseEvent::Up { x, y, at_ms }),
        _ => None,
    }
}

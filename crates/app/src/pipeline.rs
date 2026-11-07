use std::sync::mpsc;
use std::thread;

use application::auto_copy::AutoCopy;
use application::ports::{Clipboard, Feedback, Foreground};
use domain::selection::GestureTracker;
use infrastructure::mouse_hook;

pub fn start<C, F, B>(auto_copy: AutoCopy<C, F, B>)
where
    C: Clipboard + 'static,
    F: Foreground + 'static,
    B: Feedback + 'static,
{
    let (events, incoming) = mpsc::channel();
    mouse_hook::install(events);
    thread::spawn(move || {
        let mut tracker = GestureTracker::default();
        for event in incoming {
            if tracker.feed(event).is_some() {
                auto_copy.on_gesture();
            }
        }
    });
}

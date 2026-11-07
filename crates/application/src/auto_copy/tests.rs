use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

use domain::exclusions::ExcludedPrograms;
use domain::settings::Settings;

use super::AutoCopy;
use crate::ports::{Clipboard, Feedback, Foreground};
use crate::session::Session;

#[derive(Default)]
struct Probe {
    copies: AtomicU32,
    sequence: AtomicU32,
    feedback: Mutex<Vec<(bool, bool)>>,
}

struct FakeClipboard {
    probe: Arc<Probe>,
    text: bool,
    changes_on_copy: bool,
}

impl Clipboard for FakeClipboard {
    fn sequence(&self) -> u32 {
        self.probe.sequence.load(Ordering::SeqCst)
    }

    fn has_text(&self) -> bool {
        self.text
    }

    fn send_copy(&self) {
        self.probe.copies.fetch_add(1, Ordering::SeqCst);
        if self.changes_on_copy {
            self.probe.sequence.fetch_add(1, Ordering::SeqCst);
        }
    }
}

struct FakeForeground(Option<String>);

impl Foreground for FakeForeground {
    fn program(&self) -> Option<String> {
        self.0.clone()
    }
}

struct FakeFeedback(Arc<Probe>);

impl Feedback for FakeFeedback {
    fn copied(&self, toast: bool, sound: bool) {
        self.0.feedback.lock().unwrap().push((toast, sound));
    }
}

fn session_with(settings: Settings, enabled: bool) -> Arc<Session> {
    let session = Arc::new(Session::new(settings));
    if !enabled {
        session.toggle();
    }
    session
}

fn excluding(name: &str) -> Settings {
    let mut excluded = ExcludedPrograms::default();
    excluded.add(name);
    Settings {
        excluded,
        ..Settings::default()
    }
}

fn auto_copy(
    probe: &Arc<Probe>,
    program: Option<&str>,
    session: Arc<Session>,
    text: bool,
    changes_on_copy: bool,
) -> AutoCopy<FakeClipboard, FakeForeground, FakeFeedback> {
    AutoCopy::new(
        FakeClipboard {
            probe: Arc::clone(probe),
            text,
            changes_on_copy,
        },
        FakeForeground(program.map(str::to_owned)),
        FakeFeedback(Arc::clone(probe)),
        session,
    )
}

#[test]
fn successful_copy_gives_feedback_once() {
    let probe = Arc::new(Probe::default());
    let session = session_with(Settings::default(), true);
    auto_copy(&probe, Some("notepad.exe"), session, true, true).on_gesture();
    assert_eq!(*probe.feedback.lock().unwrap(), vec![(true, true)]);
}

#[test]
fn disabled_session_does_nothing() {
    let probe = Arc::new(Probe::default());
    let session = session_with(Settings::default(), false);
    auto_copy(&probe, Some("notepad.exe"), session, true, true).on_gesture();
    assert_eq!(probe.copies.load(Ordering::SeqCst), 0);
    assert!(probe.feedback.lock().unwrap().is_empty());
}

#[test]
fn excluded_program_does_nothing() {
    let probe = Arc::new(Probe::default());
    let session = session_with(excluding("explorer.exe"), true);
    auto_copy(&probe, Some("explorer.exe"), session, true, true).on_gesture();
    assert_eq!(probe.copies.load(Ordering::SeqCst), 0);
    assert!(probe.feedback.lock().unwrap().is_empty());
}

#[test]
fn unchanged_sequence_gives_no_feedback() {
    let probe = Arc::new(Probe::default());
    let session = session_with(Settings::default(), true);
    auto_copy(&probe, None, session, true, false).on_gesture();
    assert!(probe.feedback.lock().unwrap().is_empty());
}

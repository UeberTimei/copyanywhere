pub trait Clipboard: Send + Sync {
    fn sequence(&self) -> u32;
    fn has_text(&self) -> bool;
    fn send_copy(&self);
}

pub trait Foreground: Send + Sync {
    fn program(&self) -> Option<String>;
}

pub trait Feedback: Send + Sync {
    fn copied(&self, toast: bool, sound: bool);
}

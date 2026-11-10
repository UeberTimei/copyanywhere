mod exe_name;
mod http;
pub mod release_feed;
pub mod self_update;
pub mod settings_file;
mod tone;

#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod win;

#[cfg(unix)]
pub use unix::{
    autostart, clipboard, file_dialog, foreground, keyboard, mouse_hook, sound, working_set,
};
#[cfg(windows)]
pub use win::{
    autostart, clipboard, file_dialog, foreground, keyboard, mouse_hook, sound, working_set,
};

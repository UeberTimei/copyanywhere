use enigo::Direction::{Click, Press, Release};
use enigo::{Direction, Enigo, Key, Keyboard, Settings};

#[cfg(target_os = "macos")]
const MODIFIER: Key = Key::Meta;
#[cfg(not(target_os = "macos"))]
const MODIFIER: Key = Key::Control;

const COPY: [(Key, Direction); 3] = [
    (MODIFIER, Press),
    (Key::Unicode('c'), Click),
    (MODIFIER, Release),
];

pub fn send_copy_shortcut() {
    let Ok(mut enigo) = Enigo::new(&Settings::default()) else {
        return;
    };
    for (key, direction) in COPY {
        let _ = enigo.key(key, direction);
    }
}

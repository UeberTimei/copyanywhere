use super::copy_inputs;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    INPUT_KEYBOARD, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, VK_C, VK_CONTROL,
};

#[test]
fn copy_inputs_are_keyboard_events() {
    let inputs = copy_inputs();
    assert_eq!(inputs.map(|input| input.r#type), [INPUT_KEYBOARD; 4]);
}

#[test]
fn copy_inputs_press_ctrl_c_then_release_c_and_ctrl() {
    let keys = copy_inputs().map(|input| unsafe { input.Anonymous.ki });
    assert_eq!(
        keys.map(|key| key.wVk),
        [VK_CONTROL, VK_C, VK_C, VK_CONTROL]
    );
    assert_eq!(
        keys.map(|key| key.dwFlags),
        [
            KEYBD_EVENT_FLAGS(0),
            KEYBD_EVENT_FLAGS(0),
            KEYEVENTF_KEYUP,
            KEYEVENTF_KEYUP
        ]
    );
}

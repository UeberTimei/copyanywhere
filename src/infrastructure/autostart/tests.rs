use super::{is_enabled_in, set_in};
use winreg::RegKey;
use winreg::enums::HKEY_CURRENT_USER;

const TEST_KEY: &str = r"Software\CopyAnywhereTest";
const COMMAND: &str = r#""C:\Apps\copyanywhere.exe""#;

struct TestKeyGuard;

impl Drop for TestKeyGuard {
    fn drop(&mut self) {
        let _ = RegKey::predef(HKEY_CURRENT_USER).delete_subkey_all(TEST_KEY);
    }
}

#[test]
fn enable_disable_round_trip_and_disable_when_absent() {
    let _guard = TestKeyGuard;
    set_in(TEST_KEY, false, COMMAND).unwrap();
    assert!(!is_enabled_in(TEST_KEY));
    set_in(TEST_KEY, true, COMMAND).unwrap();
    assert!(is_enabled_in(TEST_KEY));
    set_in(TEST_KEY, false, COMMAND).unwrap();
    assert!(!is_enabled_in(TEST_KEY));
}

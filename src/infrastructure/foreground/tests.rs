use super::exe_name;

#[test]
fn exe_name_strips_windows_directories() {
    assert_eq!(exe_name(r"C:\Program Files\App\Notepad.exe"), "notepad.exe");
}

#[test]
fn exe_name_strips_forward_slash_directories() {
    assert_eq!(exe_name("D:/tools/Code.EXE"), "code.exe");
}

#[test]
fn exe_name_keeps_bare_file_name_lowercase() {
    assert_eq!(exe_name("Firefox.exe"), "firefox.exe");
}

#[test]
fn exe_name_of_empty_path_is_empty() {
    assert_eq!(exe_name(""), "");
}

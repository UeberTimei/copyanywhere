use domain::program::Program;

use crate::exe_name::exe_name;

// shortcut: no friendly name outside Windows, the picker shows the file name, read .desktop / Info.plist names if that is not enough
pub fn describe(path: &str) -> Program {
    Program {
        exe: exe_name(path),
        title: None,
    }
}

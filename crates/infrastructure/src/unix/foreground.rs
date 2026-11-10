use std::collections::BTreeMap;
use std::process::Command;

use active_win_pos_rs::get_active_window;
use domain::program::Program;

use super::program_title::describe;
use crate::exe_name::exe_name;

pub fn foreground_program() -> Option<String> {
    let window = get_active_window().ok()?;
    Some(exe_name(&window.process_path.to_string_lossy()))
}

// shortcut: lists every process of the user, not only the ones with windows, filter by window if the picker gets noisy
pub fn running_programs() -> Vec<Program> {
    let Ok(output) = Command::new("ps").args(["-x", "-o", "comm="]).output() else {
        return Vec::new();
    };
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|path| describe(path.trim()))
        .filter(|program| !program.exe.is_empty())
        .map(|program| (program.exe.clone(), program))
        .collect::<BTreeMap<_, _>>()
        .into_values()
        .collect()
}

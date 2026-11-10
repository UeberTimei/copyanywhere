use domain::program::Program;

use super::program_title::describe;

/// Asks the user for an executable and returns it, or `None` if cancelled.
pub fn pick_program() -> Option<Program> {
    let path = rfd::FileDialog::new().pick_file()?;
    Some(describe(&path.to_string_lossy()))
}

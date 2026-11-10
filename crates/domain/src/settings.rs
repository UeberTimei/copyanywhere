use serde::{Deserialize, Serialize};

use super::exclusions::ExcludedPrograms;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub launch_at_login: bool,
    pub sound_enabled: bool,
    pub toast_enabled: bool,
    pub check_updates: bool,
    pub skipped_version: String,
    pub excluded: ExcludedPrograms,
}

impl Default for Settings {
    fn default() -> Self {
        let mut excluded = ExcludedPrograms::default();
        excluded.add("explorer.exe");
        Self {
            launch_at_login: false,
            sound_enabled: true,
            toast_enabled: true,
            check_updates: true,
            skipped_version: String::new(),
            excluded,
        }
    }
}

#[cfg(test)]
mod tests;

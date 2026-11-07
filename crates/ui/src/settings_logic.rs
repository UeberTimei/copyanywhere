use std::collections::BTreeMap;

use domain::exclusions::ExcludedPrograms;
use domain::settings::Settings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Feature {
    LaunchAtLogin,
    Sound,
    Notification,
}

pub fn is_on(settings: &Settings, feature: Feature) -> bool {
    match feature {
        Feature::LaunchAtLogin => settings.launch_at_login,
        Feature::Sound => settings.sound_enabled,
        Feature::Notification => settings.toast_enabled,
    }
}

pub fn apply_toggle(settings: &mut Settings, feature: Feature) {
    let flag = match feature {
        Feature::LaunchAtLogin => &mut settings.launch_at_login,
        Feature::Sound => &mut settings.sound_enabled,
        Feature::Notification => &mut settings.toast_enabled,
    };
    *flag = !*flag;
}

pub fn pickable_programs(running: Vec<String>, excluded: &ExcludedPrograms) -> Vec<String> {
    let unique: BTreeMap<String, String> = running
        .into_iter()
        .map(|name| name.trim().to_owned())
        .filter(|name| !name.is_empty() && !excluded.contains(name))
        .map(|name| (name.to_lowercase(), name))
        .collect();
    unique.into_values().collect()
}

#[cfg(test)]
mod tests;

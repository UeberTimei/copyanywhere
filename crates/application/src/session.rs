use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

use domain::settings::Settings;

pub struct Session {
    enabled: AtomicBool,
    settings: RwLock<Settings>,
}

impl Session {
    pub fn new(settings: Settings) -> Self {
        Self {
            enabled: AtomicBool::new(true),
            settings: RwLock::new(settings),
        }
    }

    pub fn toggle(&self) -> bool {
        !self.enabled.fetch_xor(true, Ordering::SeqCst)
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::SeqCst)
    }

    pub fn settings(&self) -> Settings {
        self.read().clone()
    }

    pub fn update(&self, change: impl FnOnce(&mut Settings)) {
        change(&mut self.write());
    }

    fn read(&self) -> RwLockReadGuard<'_, Settings> {
        self.settings.read().unwrap_or_else(PoisonError::into_inner)
    }

    fn write(&self) -> RwLockWriteGuard<'_, Settings> {
        self.settings
            .write()
            .unwrap_or_else(PoisonError::into_inner)
    }
}

#[cfg(test)]
mod tests;

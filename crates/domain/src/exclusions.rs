use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExcludedPrograms(Vec<String>);

impl ExcludedPrograms {
    pub fn add(&mut self, name: &str) -> bool {
        let key = normalize(name);
        if key.is_empty() || self.contains(&key) {
            return false;
        }
        self.0.push(key);
        true
    }

    pub fn remove(&mut self, name: &str) {
        let key = normalize(name);
        self.0.retain(|item| normalize(item) != key);
    }

    pub fn contains(&self, name: &str) -> bool {
        let key = normalize(name);
        self.iter().any(|item| normalize(item) == key)
    }

    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.0.iter().map(String::as_str)
    }
}

fn normalize(name: &str) -> String {
    name.trim().to_lowercase()
}

#[cfg(test)]
mod tests;

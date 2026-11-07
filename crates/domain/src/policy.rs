#![allow(dead_code)]

use super::exclusions::ExcludedPrograms;

pub fn should_copy(enabled: bool, program: Option<&str>, excluded: &ExcludedPrograms) -> bool {
    enabled && !program.is_some_and(|name| excluded.contains(name))
}

#[cfg(test)]
mod tests;

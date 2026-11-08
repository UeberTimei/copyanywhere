/// A program as the user sees it: the executable name plus its human-readable product name, when known.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub exe: String,
    pub title: Option<String>,
}

impl Program {
    pub fn bare(exe: &str) -> Self {
        Self {
            exe: exe.to_owned(),
            title: None,
        }
    }

    /// `Title - exe`, or just `exe` when there is no title worth showing.
    pub fn label(&self) -> String {
        match self.title.as_deref().map(str::trim) {
            Some(title) if !title.is_empty() && !title.eq_ignore_ascii_case(&self.exe) => {
                format!("{title} - {}", self.exe)
            }
            _ => self.exe.clone(),
        }
    }
}

#[cfg(test)]
mod tests;
